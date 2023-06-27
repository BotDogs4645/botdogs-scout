use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div>
            <Navbar/>
        </div>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex h-screen w-16 flex-col justify-between border-e bg-white">
          <div>
            <div class="inline-flex h-16 w-16 items-center justify-center">
            </div>
          </div>
        </div>
    }
}