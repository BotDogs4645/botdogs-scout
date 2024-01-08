use leptos::*;

#[path = "../../lib/navbar.rs"]
mod navbar;
use navbar::*;

#[path = "./info.rs"]
mod info;
use info::*;

#[path = "../../lib/tba.rs"]
mod tba;
use tba::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    
    let matches = get_matches("4645".to_string(),"2023ilch".to_string()).await.unwrap_or(vec![
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
        ]
    );


    view! { cx,
        <div class="flex flex-row bg-gray-100 h-screen">
            <Navbar/>
            <div class="flex flex-row p-4 flex-grow">
                <div class="p-6 flex-grow">
                    <TeamCard team_number="4645".to_string()/>
                </div>
                <div class="p-6 flex-grow">
                    <RankCard team="4645".to_string()/>
                </div>
                <div class="p-6 flex-grow">
                    <MatchCard event_code="4645".to_string()/>
                </div>
            </div>
            <div>
                <table>
                </table>
            </div>
        </div>
    }
}
