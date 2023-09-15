use leptos::*;

#[path = "../../lib/navbar.rs"]
mod navbar;
use navbar::*;

#[path = "./info.rs"]
mod info;
use info::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="flex flex-row bg-gray-100 h-screen">
            <Navbar/>
            <div class="flex flex-row p-4 flex-grow">
                <div class="p-6 flex-grow"><TeamCard team_number="2012".to_string()/></div>
                <div class="p-6 flex-grow"><RankCard team="4645".to_string()/></div>
                <div class="p-6 flex-grow"><RankCard team="4645".to_string()/></div>
            </div>
        </div>
    }
}
