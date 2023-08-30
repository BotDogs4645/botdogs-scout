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

#[derive(Serialize, Deserialize)]
struct TeamInfo {
    key: String,
    team_number: i32,
    nickname: String,
    name: String,
    city: String,
    state_prov: String,
    country: String,
    address: Option<String>,
    postal_code: String,
    gmaps_place_id: Option<String>,
    gmaps_url: Option<String>,
    lat: Option<f32>,
    lng: Option<f32>,
    location_name: Option<String>,
    website: String,
    rookie_year: i32,
    motto: Option<String>,
    home_championship: Option<HomeChampionship>,
}
#[derive(Serialize, Deserialize)]
struct HomeChampionship {
    year: i32,
    event_code: String,
    division_code: String,
}

pub async fn get_team_name(team_number: i32) -> Result<String, reqwest::Error> {
    let resp_deserialized = get_team_info(team_number).await?;

    Ok((resp_deserialized.nickname.to_string()))
}


async fn get_team_info(team_number: i32) -> Result<TeamInfo, reqwest::Error> {
    let client = Client::builder()
        .build()?
        .get(&format!("https://www.thebluealliance.com/api/v3/team/frc{team_number}", team_number=team_number))
        .header("X-TBA-Auth-Key", "hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf")
        .send()
        .await?;
    let body = client.text().await?;
    let resp_deserialized: TeamInfo = serde_json::from_str(&body).unwrap();
    Ok(resp_deserialized)
}