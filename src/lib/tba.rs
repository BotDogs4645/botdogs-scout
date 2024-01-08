use std::{env, collections::HashMap, thread, clone};
use leptos::leptos_dom::console_log;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Clone)]
struct EventRankings {
    pub rankings: Vec<Ranking>,
}
#[derive(Serialize, Deserialize, Clone)]
struct Ranking {
    pub rank: i32,
    pub team_key: String,
    pub matches_played: i32,
    pub qual_average: Option<f32>,
    pub record: Record,
    pub sort_orders: Vec<f32>,
    pub dq: i32,
}
#[derive(Serialize, Deserialize, Clone)]
struct Record {
    pub losses: i32,
    pub wins: i32,
    pub ties: i32,
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

#[derive(Serialize, Deserialize, Clone)]
struct TeamInfo {
    pub key: Option<String>,
    pub team_number: Option<i32>,
    pub nickname: Option<String>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub state_prov: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub postal_code: Option<String>,
    pub gmaps_place_id: Option<String>,
    pub gmaps_url: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub location_name: Option<String>,
    pub website: Option<String>,
    pub rookie_year: Option<i32>,
    pub motto: Option<String>,
    pub home_championship: Option<HomeChampionship>,
    pub school_name: Option<String>,
    pub Error: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
struct HomeChampionship {
    pub year: i32,
    pub event_code: String,
    pub division_code: String,
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
#[derive(Serialize, Deserialize, Clone)]
pub struct Matches {
    pub matches: Option<Vec<Match>>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Match {
    pub actual_time: Option<i32>,
    pub alliances: Option<Alliances>,
    pub comp_level: Option<String>,
    pub event_key: Option<String>,
    pub key: Option<String>,
    pub match_number: Option<i32>,
    pub post_result_time: Option<i32>,
    pub predicted_time: Option<i32>,
    pub score_breakdown: Option<ScoreBreakdown>,
    pub set_number: Option<i32>,
    pub time: Option<i32>,
    pub videos: Option<Vec<Video>>,
    pub winning_alliance: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Video {
    pub key: Option<String>,
    pub type_: Option<String>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alliances {
    pub blue: Option<Alliance>,
    pub red: Option<Alliance>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Alliance {
    pub dq_team_keys: Option<Vec<String>>,
    pub score: Option<i32>,
    pub surrogate_team_keys: Option<Vec<String>>,
    pub team_keys: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreBreakdown {
    pub blue: Option<Score>,
    pub red: Option<Score>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Score {
    pub activation_bonus_achieved: bool,
    pub adjust_points: i32,
    pub auto_bridge_state: String,
    pub auto_charge_station_points: i32,
    pub auto_charge_station_robot1: String,
    pub auto_charge_station_robot2: String,
    pub auto_charge_station_robot3: String,
    pub auto_community: AutoCommunity,
    pub auto_docked: bool,
    pub auto_game_piece_count: i32,
    pub auto_game_piece_points: i32,
    pub auto_mobility_points: i32,
    pub auto_points: i32,
    pub coop_game_piece_count: i32,
    pub coopertition_criteria_met: bool,
    pub end_game_bridge_state: String,
    pub end_game_charge_station_points: i32,
    pub end_game_charge_station_robot1: String,
    pub end_game_charge_station_robot2: String,
    pub end_game_charge_station_robot3: String,
    pub end_game_park_points: i32,
    pub foul_count: i32,
    pub foul_points: i32,
    pub link_points: i32,
    pub links: Vec<Link>,
    pub mobility_robot1: String,
    pub mobility_robot2: String,
    pub mobility_robot3: String,
    pub rp: i32,
    pub sustainability_bonus_achieved: bool,
    pub tech_foul_count: i32,
    pub teleop_community: TeleopCommunity,
    pub teleop_game_piece_count: i32,
    pub teleop_game_piece_points: i32,
    pub teleop_points: i32,
    pub total_charge_station_points: i32,
    pub total_points: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AutoCommunity {
    pub b: Vec<String>,
    pub m: Vec<String>,
    pub t: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Link {
    pub nodes: Vec<i32>,
    pub row: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TeleopCommunity {
    pub b: Vec<String>,
    pub m: Vec<String>,
    pub t: Vec<String>,
}

async fn get_match_vec(team_number: String, event_code: String) -> Result<String, reqwest::Error>{
    let client = Client::builder()
    .build()?
    .get(&format!("https://www.thebluealliance.com/api/v3/team/frc{team_number}/event/{event_code}/matches", team_number=team_number, event_code=event_code))
    .header("X-TBA-Auth-Key", "hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf")
    .send()
    .await?;

    Ok(client.text().await?)
}
 

pub async fn get_matches(team_number: String, event_code: String) -> Matches {

    let body = get_match_vec(team_number, event_code).await.unwrap_or(json!([]).to_string());

    let resp_deserialized: Matches = serde_json::from_str(&body).unwrap_or(Matches { matches: None });
    resp_deserialized
}

pub async fn get_current_match(team_number: String, event_code: String) -> Match {
    let matches = get_matches(team_number, event_code).await;
    let matches_arr = matches.matches.unwrap_or(vec![
        Match {
            actual_time: None,
            alliances: None,
            comp_level: None,
            event_key: None,
            key: None,
            match_number: None,
            post_result_time: None,
            predicted_time: None,
            score_breakdown: None,
            set_number: None,
            time: None,
            videos: None,
            winning_alliance: None 
        }
    ]); 

    matches_arr[if matches_arr.len() == 0 {
        0
    } else {
        matches_arr.len() - 1
    }].clone()
}

