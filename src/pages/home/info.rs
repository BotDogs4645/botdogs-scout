use http::Request;
use leptos::*;
use std::{env, collections::HashMap, sync::Arc};

#[path = "../../lib/tba.rs"]
mod tba;
use tba::*;

#[component]
pub fn RankCard(cx: Scope, team: String) -> impl IntoView {

  let team = "frc".to_string() + &team;

  let tba_rankings = create_resource(cx, || (), |_| async move {
    get_match_rankings("2023ilch".to_string()).await.unwrap_or(HashMap::new())
  });
  let result = move || {
    tba_rankings
      .read(cx)
      .map(|value| format!("Team Rank: {rank}", rank=value.get(&team).unwrap_or(&-1)))
      .unwrap_or_else(|| "Loading...".into())
  };
  
  view! {
    cx,
    <div class="bg-white rounded-lg justify-center text-center shadow-lg">
      <div class="p-4">
        <h1 class="text-2xl"><b>"Rankings"</b></h1>
        {result}
      </div>
    </div>
  }
}
#[component]
pub fn TeamCard(cx: Scope, team_number: String) -> impl IntoView {

  // i know the variable names are garbage, its bc of stupid deep copy vs shallow copy shit and bc
  // of the async closure moving the memory adress to a different thread :(

  let team_id = team_number.clone();

  let team_name = create_resource(cx, || (),   move |_| {
    let team_number = team_number.clone();
    async move {
      get_team_name(team_number).await.unwrap_or("Loading".to_string())
    }
  });
  

  let name_val = move || {
    team_name
      .read(cx)
      .map(|value| format!("{team:?}", team=value))
      .unwrap_or_else(|| "Loading...".into())
  };

  view! {
    cx,
    <div class="bg-white rounded-lg justify-center text-center shadow-lg">
      <div class="p-4">
        <h1 class="text-2xl"><b>"My Team"</b></h1>
        {format!("#{num}: ", num=team_id)}
        {name_val} 
      </div>
    </div>
  }
}

pub fn MatchCard(cx: Scope, event_code: String) -> impl IntoView {

  let curr_match_future = create_resource(cx, || (), move |_| {
    get_current_match()
  });

  let curr_match = move || {
    curr_match_future
      .read(cx)
      .map(|value| format!())
      .unwrap_or_else(|| "Loading...".into())
  };

    view! {
      cx,
      <div class="bg-white rounded-lg justify-center text-center shadow-lg">
        <div class="p-4">
          <h1 class="text-2xl"><b>"Match"</b></h1>
          {curr_match}"/"{total_matches}
        </div>
      </div>
    }
}
