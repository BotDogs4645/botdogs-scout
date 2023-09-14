use http::Request;
use leptos::*;
use std::{env, collections::HashMap};

#[path = "../../lib/tba.rs"]
mod tba;
use tba::*;

#[component]
pub fn RankCard(cx: Scope) -> impl IntoView {

  let tba_rankings = create_resource(cx, || (), |_| async move {
    get_match_rankings("2023ilch".to_string()).await.unwrap_or(HashMap::new())
  });
  let result = move || {
    tba_rankings
      .read(cx)
      .map(|value| format!("TBA Status: {value:?}"))
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
pub fn TeamCard(cx: Scope, team_number: i32) -> impl IntoView {

  let team_name = create_resource(cx, || (), move |_| async move  {
    get_team_name(team_number).await.unwrap_or("Loading".into())
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
        {format!("#{num}: ", num=team_number)}
        {name_val} 
      </div>
    </div>
  }
}

pub fn MatchCard(cx: Scope) -> impl IntoView {

    view! {
      cx,
      
    }
}
