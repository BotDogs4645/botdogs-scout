use leptos::*;

#[path = "../../lib/navbar.rs"] mod navbar;
use navbar::*;

mod info;


#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="grid ">
            <Navbar/>
        </div>
        
    }
}
