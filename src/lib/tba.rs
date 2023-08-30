use std::env;

use leptos::leptos_dom::console_log;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TbaStatus {
    current_season: i32,
    max_season: i32,
    is_datafeed_down: bool,
    down_events: Vec<String>,
    ios: TbaStatusAppVersion,
    android: TbaStatusAppVersion,
}
#[derive(Serialize, Deserialize)]
struct TbaStatusAppVersion {
    min_app_version: i32,
    latest_app_version: i32,
}

pub async fn get_tba_status() -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .build()?
        .get("https://www.thebluealliance.com/api/v3/status")
        .header("X-TBA-Auth-Key", "hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf")
        .send()
        .await?;
    
    let body = client.text().await?;
    let resp_deserialized:TbaStatus = serde_json::from_str(&body).unwrap();

    log::info!("TBA Status: {:?}", resp_deserialized.is_datafeed_down.to_string());

    Ok(resp_deserialized.is_datafeed_down.to_string())
}


pub async fn get_team_info(team_number: i32) -> Result<String, reqwest::Error> {
    let client = Client::builder()
        .build()?
        .get(&format!("https://www.thebluealliance.com/api/v3/team/frc{team_number}", team_number=team_number))
        .header("X-TBA-Auth-Key", "hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf")
        .send()
        .await?;
    
    let body = client.text().await?;
    let resp_deserialized:TbaStatus = serde_json::from_str(&body).unwrap();

    log::info!("TBA Status: {:?}", resp_deserialized.is_datafeed_down.to_string());

    Ok(resp_deserialized.is_datafeed_down.to_string())
}