#[path = "../../lib/navbar.rs"] mod navbar;
use leptos::*;
use navbar::*;

#[component]
pub fn MyTeamPage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="flex flex-row bg-gray-100 h-screen">
            <Navbar/>
            
        </div>
    }
}
