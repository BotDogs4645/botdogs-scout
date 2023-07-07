use leptos::*;

#[path = "../../lib/navbar.rs"] mod navbar;
use navbar::*;

mod info;


#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="grid grid-cols-4 gap-3 bg-gray-100">
            <Navbar/>

            <div class="bg-white rounded-xl">
            </div>
        </div>
    }
}
