use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoPosition {
    pub lat: f64,
    pub lon: f64,
}

#[tauri::command]
pub async fn get_ip_geoposition() -> Result<GeoPosition, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client
        .get("https://ipwho.is/?fields=success,message,latitude,longitude")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json = resp.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;

    if json["success"] == false {
        return Err(json["message"]
            .as_str()
            .unwrap_or("geolocation failed")
            .to_string());
    }

    Ok(GeoPosition {
        lat: json["latitude"].as_f64().ok_or("missing latitude")?,
        lon: json["longitude"].as_f64().ok_or("missing longitude")?,
    })
}