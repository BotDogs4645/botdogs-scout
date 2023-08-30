use http::Request;
use leptos::*;
use std::env;

#[path = "../../lib/tba.rs"]
mod tba;
use tba::*;

#[component]
pub fn RankCard(cx: Scope) -> impl IntoView {

  let tba_rankings = create_resource(cx, || (), |_| async move {
    get_tba_status().await.unwrap()
  });
  let result = move || {
    tba_rankings
      .read(cx)
      .map(|value| format!("TBA Status: {value:?}"))
      .unwrap_or_else(|| "Loading...".into())
  };
  
  view! {
    cx,
    <div class="bg-white rounded-lg justify-center text-center">
      <div class="p-4">
        <h1 class="text-2xl">"Rankings"</h1>
        {result}
      </div>
    </div>
  }
}

pub fn TeamCard(cx: Scope) -> impl IntoView {

    view! {
      cx,
      
    }
}

pub fn MatchCard(cx: Scope) -> impl IntoView {

    view! {
      cx,
      
    }
}
