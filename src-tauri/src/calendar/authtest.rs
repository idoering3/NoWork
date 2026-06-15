use std::io::BufReader;

use chrono::{Datelike, Duration, TimeZone};
use ical::{IcalParser, property::Property};
use reqwest::{Client, Method};
use roxmltree::Document;
use serde::{Deserialize, Serialize};
use rrule::{RRuleSet, Tz};

#[derive(Serialize, Deserialize, Debug)]
pub struct CalendarEvent {
    pub summary: String,
    pub start: String,
    pub end: String,
    pub uid: String,
    pub description: Option<String>,
    pub location: Option<String>,
}

#[tauri::command]
pub async fn test_auth(email: &str) -> Result<Vec<CalendarEvent>, String> {
    
    // load credentials
    let pass = load_credentials(email)?;

    println!("testing auth");
    // this gets caldav user root identity, required for the actual url
    let client = Client::new();

    let method = Method::from_bytes(b"PROPFIND").map_err(|e| e.to_string())?;

    let response = client
        .request(method.clone(), "https://caldav.icloud.com/.well-known/caldav")
        .basic_auth(
            email,
            Some(pass.clone()),
        )
        .header("Depth", "0")
        .body(
            r#"<?xml version="1.0" encoding="utf-8" ?>
            <propfind xmlns="DAV:">
                <prop>
                    <current-user-principal />
                </prop>
            </propfind>"#,
        )
        .send()
        .await.map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    let doc = Document::parse(&text).map_err(|e| e.to_string())?;

    let href = doc
        .descendants()
        .find(|n| n.has_tag_name("href"))
        .and_then(|n| n.text())
        .ok_or("no href found")?;
    // more testing of the url we received
    let url = format!("https://caldav.icloud.com{}", href);

    let body = r#"<?xml version="1.0" encoding="utf-8" ?>
        <propfind xmlns="DAV:">
        <prop>
            <calendar-home-set xmlns="urn:ietf:params:xml:ns:caldav"/>
        </prop>
        </propfind>"#;

    let response2 = client.request(method.clone(), url)
        .basic_auth(
                email,
                Some(pass.clone()),
            ).header("Depth", "0").body(body).send().await.map_err(|e| e.to_string())?;


    let text = response2.text().await.map_err(|e| e.to_string())?;

    // this is the thing that will get us the actual apple calendar link
    let doc2 = Document::parse(&text).map_err(|e: roxmltree::Error| e.to_string())?;

    let href = doc2
        .descendants()
        .filter(|n| n.tag_name().name() == "href")
        .find(|n| {
            n.text()
                .unwrap_or("")
                .contains("calendars")
        })
        .and_then(|n| n.text())
        .map(|t| t.trim())
        .ok_or("no calendars href found")?;

    // this should be our actual calendars url
    let url = format!("{}", href);

    // now we need to get a specific calendar
    let body = r#"<?xml version="1.0" encoding="utf-8" ?>
        <propfind xmlns="DAV:">
            <prop>
                <displayname />
                <resourcetype />
            </prop>
        </propfind>"#;

    let response3 = client.request(method.clone(), url)
        .basic_auth(
                email,
                Some(pass.clone()),
            ).header("Depth", "1").body(body).send().await.map_err(|e| e.to_string())?;
    
    let text = response3.text().await.map_err(|e| e.to_string())?;

    let doc3 = Document::parse(&text).map_err(|e: roxmltree::Error| e.to_string())?;


    let href = doc3
        .descendants()
        .filter(|n| n.tag_name().name() == "href")
        .skip(1) // Skip the first element
        .map(|n| {n.text().unwrap_or("").trim().to_string()})
        .collect::<Vec<_>>();

    // ugh, what a mess
    let all_events = fetch_all_calendars(
        &client,
        "https://caldav.icloud.com",
        &href,
        email.to_string(),
        pass.to_string()
    )
    .await.map_err(|e| e.to_string())?;

    Ok(all_events)
}

fn get_prop( props: &[Property], name: &str ) -> Option<String> { 
    props.iter().find(|p| p.name == name).and_then(|p| p.value.clone()) 
}

fn parse_events(calendar_data: &str) -> Vec<CalendarEvent> {
    let reader = BufReader::new(calendar_data.as_bytes());
    let parser = IcalParser::new(reader);
    let mut events = Vec::new();

    for calendar in parser.flatten() {
        for event in calendar.events {
            let props = &event.properties;

            // Grab raw property lines — rrule crate needs the full
            // "DTSTART;TZID=...:value" form, not just the value.
            let dtstart_raw = get_raw_prop(props, "DTSTART");
            // let dtend_raw   = get_raw_prop(props, "DTEND");
            let rrule_raw   = get_raw_prop(props, "RRULE");

            let summary     = get_prop(props, "SUMMARY").unwrap_or_default();
            let uid         = get_prop(props, "UID").unwrap_or_default();
            let description = get_prop(props, "DESCRIPTION");
            let location    = get_prop(props, "LOCATION");

            let (Some(start), Some(end)) = (
                get_prop(props, "DTSTART"),
                get_prop(props, "DTEND"),
            ) else { continue };

            match (dtstart_raw, rrule_raw) {
                // ── Recurring event ──────────────────────────────────────────
                (Some(dtstart_line), Some(rrule_line)) => {
                    // Compute the event duration from the first occurrence
                    // so we can apply it to every expanded date.
                    let duration = match event_duration(&start, &end) {
                        Some(d) => d,
                        None => continue,
                    };

                    // The rrule crate expects one string with DTSTART on the
                    // first line and RRULE on the second.
                    let rrule_str = format!("{}\n{}", dtstart_line, rrule_line);

                    let Ok(set) = rrule_str.parse::<RRuleSet>() else {
                        // Fallback: treat as a single event if parsing fails.
                        events.push(CalendarEvent {
                            summary: summary.clone(), start, end,
                            uid: uid.clone(), description: description.clone(),
                            location: location.clone(),
                        });
                        continue;
                    };

                    let now = chrono::Utc::now();
                    let start_of_week = (now - chrono::Duration::days(
                        now.weekday().num_days_from_sunday() as i64
                    )).date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
                    let end_of_week = start_of_week + chrono::Duration::days(7);
                    let start_of_week = start_of_week.with_timezone(&Tz::UTC);
                    let end_of_week = end_of_week.with_timezone(&Tz::UTC);

                    for occurrence in set.after(start_of_week).before(end_of_week).all(50).dates {
                        let occ_end = occurrence + duration;
                        events.push(CalendarEvent {
                            summary:     summary.clone(),
                            uid:         uid.clone(),
                            description: description.clone(),
                            location:    location.clone(),
                            start: occurrence.to_rfc3339(),
                            end:   occ_end.to_rfc3339(),
                        });
                    }
                }
                _ => {
                    let start_norm = normalize_ical_dt(props, "DTSTART").unwrap_or(start);
                    let end_norm   = normalize_ical_dt(props, "DTEND").unwrap_or(end);
                    events.push(CalendarEvent {
                        summary, start: start_norm, end: end_norm, uid, description, location,
                    });
                }
            }
        }
    }
    events
}

/// Returns the full raw property line e.g. "DTSTART;TZID=Europe/Paris:20260119T110000"
/// The rrule crate needs the TZID param to resolve the timezone correctly.
fn get_raw_prop(props: &[Property], name: &str) -> Option<String> {
    props.iter().find(|p| p.name == name).map(|p| {
        let params = p.params.as_ref()
            .map(|ps| {
                ps.iter()
                  .flat_map(|(k, vs)| vs.iter().map(move |v| format!(";{}={}", k, v)))
                  .collect::<String>()
            })
            .unwrap_or_default();
        format!("{}{}:{}", p.name, params, p.value.as_deref().unwrap_or(""))
    })
}

fn event_duration(start: &str, end: &str) -> Option<Duration> {
    let parse = |s: &str| {
        // Strip trailing Z if present, then parse as naive — we only need the delta
        chrono::NaiveDateTime::parse_from_str(s.trim_end_matches('Z'), "%Y%m%dT%H%M%S").ok()
    };
    let s = parse(start)?;
    let e = parse(end)?;
    Some(e.signed_duration_since(s))
}

fn normalize_ical_dt(props: &[Property], name: &str) -> Option<String> {
    let prop = props.iter().find(|p| p.name == name)?;
    let value = prop.value.as_deref()?;
    
    // Check for TZID parameter
    let tzid = prop.params.as_ref()
        .and_then(|ps| ps.iter().find(|(k, _)| k == "TZID"))
        .and_then(|(_, vs)| vs.first())
        .map(|s| s.as_str());

    if let Some(tz_name) = tzid {
        // Parse with timezone
        let tz: chrono_tz::Tz = tz_name.parse().ok()?;
        let naive = chrono::NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%S").ok()?;
        let dt = tz.from_local_datetime(&naive).single()?;
        Some(dt.to_rfc3339())
    } else {
        // No TZID — trailing Z means UTC, no Z is ambiguous but treat as UTC
        let naive = chrono::NaiveDateTime::parse_from_str(
            value.trim_end_matches('Z'), "%Y%m%dT%H%M%S"
        ).ok()?;
        Some(chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(naive, chrono::Utc)
            .to_rfc3339())
    }
}


use futures::future::join_all;

use crate::calendar::cal_credentials::load_credentials;

pub async fn fetch_all_calendars(
    client: &Client,
    base_url: &str,
    hrefs: &[String],
    email: String,
    pass: String
) -> Result<Vec<CalendarEvent>, Box<dyn std::error::Error>> {
    let method2 = Method::from_bytes(b"REPORT")?;


    let requests = hrefs.iter().map(|href| {
        let client = client.clone();
        let email = email.clone();
        let pass = pass.clone();
        let url = format!("{base_url}{href}");
        let method = method2.clone();
        
        // filtering so that we only get events for the week
        let now = chrono::Utc::now();
        let start_of_week = now - chrono::Duration::days(now.weekday().num_days_from_sunday() as i64);
        let end_of_week = start_of_week + chrono::Duration::days(7);

        let start_str = start_of_week.format("%Y%m%dT000000Z").to_string();
        let end_str = end_of_week.format("%Y%m%dT000000Z").to_string();

        let body = format!(r#"<?xml version="1.0" encoding="utf-8" ?>
                <c:calendar-query
                    xmlns:d="DAV:"
                    xmlns:c="urn:ietf:params:xml:ns:caldav">
                    <d:prop>
                        <d:getetag />
                        <c:calendar-data />
                    </d:prop>
                    <c:filter>
                        <c:comp-filter name="VCALENDAR">
                            <c:comp-filter name="VEVENT">
                                <c:time-range start="{}" end="{}"/>
                            </c:comp-filter>
                        </c:comp-filter>
                    </c:filter>
                </c:calendar-query>"#, start_str, end_str);



        async move {
            let response = client
                .request(method, &url)
                .basic_auth(email, Some(pass))
                .header("Depth", "1")
                .header("Content-Type", "application/xml")
                .body(body)
                .send()
                .await?;

            let calendar_text = response.text().await?;

            if calendar_text.trim().is_empty() {
                return Ok(vec![]);
            }

            let doc = match roxmltree::Document::parse(&calendar_text) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("XML parse error: {}", e);
                    return Ok(vec![]);
                }
            };

            let events = doc
                .descendants()
                .filter(|n| n.tag_name().name() == "calendar-data")
                .flat_map(|n| parse_events(n.text().unwrap_or("")))
                .collect();

            Ok(events)
        }
    });

let results: Vec<Result<Vec<CalendarEvent>, Box<dyn std::error::Error + Send + Sync>>> = join_all(requests).await;
    let mut all_events = Vec::new();

    for result in results {
        match result {
            Ok(events) => all_events.extend(events),
            Err(e) => {
                eprintln!("Failed to fetch calendar: {}", e);
            }
        }
    }

    Ok(all_events)
}