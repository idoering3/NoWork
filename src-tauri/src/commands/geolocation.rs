use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoPosition {
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub region: String,
    pub country: String
}

#[tauri::command]
pub async fn get_ip_geoposition() -> Result<GeoPosition, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("https://ipwho.is/?fields=success,message,latitude,longitude,city,country,region")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json = resp.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;

    println!("{:?}", json);

    if json["success"] == false {
        return Err(json["message"]
            .as_str()
            .unwrap_or("geolocation failed")
            .to_string());
    }

    Ok(GeoPosition {
        lat: json["latitude"].as_f64().ok_or("missing latitude")?,
        lon: json["longitude"].as_f64().ok_or("missing longitude")?,
        city: json["city"].as_str().ok_or("missing city")?.to_string(),
        region: json["region"].as_str().ok_or("missing region")?.to_string(),
        country: json["country"].as_str().ok_or("missing country")?.to_string()
    })
}