use std::{env, collections::HashMap};

use leptos::leptos_dom::console_log;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EventRankings {
    rankings: Vec<Ranking>,
}
#[derive(Serialize, Deserialize)]
struct Ranking {
    rank: i32,
    team_key: String,
    matches_played: i32,
    qual_average: Option<f32>,
    record: Record,
    sort_orders: Vec<f32>,
    dq: i32,
}
#[derive(Serialize, Deserialize)]
struct Record {
    losses: i32,
    wins: i32,
    ties: i32,
}

pub async fn get_match_rankings(event_code: String) -> Result<HashMap<String, i32>, reqwest::Error> {
    let mut ret: HashMap<String, i32> = HashMap::new();

    let client = Client::builder()
        .build()?
        .get(format!("https://www.thebluealliance.com/api/v3/event/{code}/rankings", code=event_code))
        .header("X-TBA-Auth-Key", "hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf")
        .send()
        .await?;

    let body = client.text().await?;

    let resp_deserialized: EventRankings = serde_json::from_str(&body).unwrap();

    for ranking in resp_deserialized.rankings {
        ret.insert(ranking.team_key, ranking.rank);
    }

    Ok(ret)

}

#[derive(Serialize, Deserialize)]
struct TeamInfo {
    key: Option<String>,
    team_number: Option<i32>,
    nickname: Option<String>,
    name: Option<String>,
    city: Option<String>,
    state_prov: Option<String>,
    country: Option<String>,
    address: Option<String>,
    postal_code: Option<String>,
    gmaps_place_id: Option<String>,
    gmaps_url: Option<String>,
    lat: Option<f32>,
    lng: Option<f32>,
    location_name: Option<String>,
    website: Option<String>,
    rookie_year: Option<i32>,
    motto: Option<String>,
    home_championship: Option<HomeChampionship>,
    school_name: Option<String>,
    Error: Option<String>,
}
#[derive(Serialize, Deserialize)]
struct HomeChampionship {
    year: i32,
    event_code: String,
    division_code: String,
}

pub async fn get_team_name(team_number: &String) -> Result<String, reqwest::Error> {
    let resp_deserialized = get_team_info(team_number).await?;

    Ok(resp_deserialized.nickname.unwrap_or_else(|| resp_deserialized.Error.unwrap()).to_string())
}


async fn get_team_info(team_number: &String) -> Result<TeamInfo, reqwest::Error> {
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