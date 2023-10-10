use std::{env, collections::HashMap, thread, clone};
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

pub async fn get_team_name(team_number: String) -> Result<String, reqwest::Error> {
    let resp_deserialized = get_team_info(team_number).await?;

    Ok(resp_deserialized.nickname.unwrap_or_else(|| resp_deserialized.Error.unwrap()).to_string())
}


async fn get_team_info(team_number: String) -> Result<TeamInfo, reqwest::Error> {
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
#[derive(Serialize, Deserialize)]
pub struct Matches {
    matches: Vec<Match>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Match {
    actual_time: Option<i32>,
    alliances: Option<Alliances>,
    comp_level: Option<String>,
    event_key: Option<String>,
    key: Option<String>,
    match_number: Option<i32>,
    post_result_time: Option<i32>,
    predicted_time: Option<i32>,
    score_breakdown: Option<ScoreBreakdown>,
    set_number: Option<i32>,
    time: Option<i32>,
    videos: Option<Vec<Video>>,
    winning_alliance: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Video {
    key: Option<String>,
    type_: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alliances {
    blue: Option<Alliance>,
    red: Option<Alliance>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alliance {
    dq_team_keys: Option<Vec<String>>,
    score: Option<i32>,
    surrogate_team_keys: Option<Vec<String>>,
    team_keys: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreBreakdown {
    blue: Option<Score>,
    red: Option<Score>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Score {
    activation_bonus_achieved: bool,
    adjust_points: i32,
    auto_bridge_state: String,
    auto_charge_station_points: i32,
    auto_charge_station_robot1: String,
    auto_charge_station_robot2: String,
    auto_charge_station_robot3: String,
    auto_community: AutoCommunity,
    auto_docked: bool,
    auto_game_piece_count: i32,
    auto_game_piece_points: i32,
    auto_mobility_points: i32,
    auto_points: i32,
    coop_game_piece_count: i32,
    coopertition_criteria_met: bool,
    end_game_bridge_state: String,
    end_game_charge_station_points: i32,
    end_game_charge_station_robot1: String,
    end_game_charge_station_robot2: String,
    end_game_charge_station_robot3: String,
    end_game_park_points: i32,
    foul_count: i32,
    foul_points: i32,
    link_points: i32,
    links: Vec<Link>,
    mobility_robot1: String,
    mobility_robot2: String,
    mobility_robot3: String,
    rp: i32,
    sustainability_bonus_achieved: bool,
    tech_foul_count: i32,
    teleop_community: TeleopCommunity,
    teleop_game_piece_count: i32,
    teleop_game_piece_points: i32,
    teleop_points: i32,
    total_charge_station_points: i32,
    total_points: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AutoCommunity {
    b: Vec<String>,
    m: Vec<String>,
    t: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    nodes: Vec<i32>,
    row: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeleopCommunity {
    b: Vec<String>,
    m: Vec<String>,
    t: Vec<String>,
}

async fn get_matches(team_number: String, event_code: String) -> Result<Matches, reqwest::Error> {
    let client = Client::builder()
        .build()?
        .get(&format!("https://www.thebluealliance.com/api/v3/team/frc{team_number}/event/{event_code}/matches", team_number=team_number, event_code=event_code))
        .header("X-TBA-Auth-Key", "hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf")
        .send()
        .await?;
    let body = client.text().await?;
    let resp_deserialized: Matches = serde_json::from_str(&body).unwrap();
    Ok(resp_deserialized)
}

async fn get_current_match(team_number: String, event_code: String) -> Result<Match, reqwest::Error> {
    let matches = get_matches(team_number, event_code).await;
    let matches = matches.unwrap_or(Matches { matches: vec![] });

    Ok(matches.matches[matches.matches.len() - 1].clone())
}