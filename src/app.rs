
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[path = "pages/home/home.rs"] mod home; 
use home::*;

#[path = "pages/auth/auth.rs"] mod auth;

#[path = "pages/my_team/my_team.rs"] mod my_team;
use my_team::*;


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/botdogs-scout.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=|cx| view!{cx, <HomePage/>}/>
                    <Route path="/my_team" view=|cx| view!{cx, <MyTeamPage/>}/>
                </Routes>
            </main>
        </Router>
    }
}
