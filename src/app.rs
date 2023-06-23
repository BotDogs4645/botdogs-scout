use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[path = "pages/home/home.rs"] mod home; 
use home::*;

#[path = "pages/auth/auth.rs"] mod auth;
use auth::*;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view!{cx, <AuthPage/>}/>
                    <Route path="/test" view=|cx| view!{cx, <HomePage/>}/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"The count is: "{count}</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <a href="/">"Back"</a>
    }
}
