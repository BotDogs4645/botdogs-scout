#[path = "../../lib/navbar.rs"] mod navbar;
use leptos::*;
use navbar::*;

#[component]
pub fn MyTeamPage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="flex flex-row bg-gray-100 h-screen">
            <div class="w-[288px]">
                <Navbar/>
            </div>
            <div class="flex flex-col flex-grow p-4">
                <div class="flex flex-row flex-grow p-4">
                    <div class="p-4">
                        <div class="rounded-lg shadow-lg bg-white p-4 flex-grow h-64">
                            <h1 class="text-6xl text-blue-900"><b>"Current Rank: -1"</b></h1>
                            <br/>
                            <h1 class="text-lg"><b>"Change of {None} from previous match"</b></h1>
                        </div>
                    </div>
                    <div class="p-4">
                        <div class="rounded-lg shadow-lg bg-white p-4 flex-grow h-64">
                            <h1 class="text-6xl text-blue-900"><b>"Average Match Score: -1"</b></h1>
                            <br/>
                            <h1 class="text-lg"><b>"Change of {None} from previous match"</b></h1>
                        </div>
                    </div>
                    <div class="p-4">
                        <div class="rounded-lg shadow-lg bg-white p-4 flex-grow h-64">
                            <h1 class="text-6xl text-blue-900"><b>"Next Match: -1"</b></h1>
                            <br/>
                            <h1 class="text-lg"><b>"{None} matches from now (in {None} minutes)"</b></h1>
                        </div>
                    </div>
                </div>
                <div class="bg-white rounded-lg h-96 text-center shadow-xl">
                    <h1><b>"{Graph not loaded because no data is present}"</b></h1>
                </div>
            </div>
        </div>
    }
}
