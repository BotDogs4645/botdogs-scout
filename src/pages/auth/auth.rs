use leptos::*;

#[component]
pub fn AuthPage(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <p>"test"</p>
            <br/>
            <a href="/test">"Click Me!"</a>
        </div>
    }
}