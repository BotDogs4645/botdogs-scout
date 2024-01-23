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
    
    let match_arr = create_resource(cx, || (), |_| async move {
        get_matches("4645".to_string(), "2023ilch".to_string()).await;
    });

    
    view! { cx,
        <div class="flex flex-row bg-gray-100 h-screen">
            <Navbar/>
            <div class="flex flex-col p-4 flex-grow">
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
                <div class="bg-white rounded-lg  flex flex-col p-4 flex-grow">
                    <h1 class="text-2xl text-center"><b>"Upcoming Matches"</b></h1>
                    <table>
                        <thead>
                            <tr>
                                <th class="w-1/2">"Red Blue Alliance Numbers"</th>
                                <th class="w-1/2">"Blue Alliance Numbers"</th>
                            </tr>
                        </thead>
                        <tbody class="h-40 overflow-y-scroll">
                            <tr>
                                <td class="bg-red-600">"---"</td>
                                <td class="bg-blue-600">"---"</td>
                            </tr>
                            <tr>
                                <td class="bg-red-600">"---"</td>
                                <td class="bg-blue-600">"---"</td>
                            </tr>
                            <tr>
                                <td class="bg-red-600">"---"</td>
                                <td class="bg-blue-600">"---"</td>
                            </tr>
                            <tr>
                                <td class="bg-red-600">"---"</td>
                                <td class="bg-blue-600">"---"</td>
                            </tr>
                            <tr>
                                <td class="bg-red-600">"---"</td>
                                <td class="bg-blue-600">"---"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    }
}
