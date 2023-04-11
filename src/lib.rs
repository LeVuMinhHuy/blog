use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod routes;
use routes::about::*;
use routes::blog::*;
use routes::error::*;
use routes::home::*;
use routes::post::*;

#[allow(non_snake_case)]
#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <div id="root">
            <Router>
                <main class="container">
                    <Routes>
                        <Route path="" view=|cx| view! { cx, <Home/> } />
                        <Route path="blog" view=|cx| view! { cx, <Blog/> } />
                        <Route path="blog/:id" view=|cx|  view! {cx, <Post /> }/>
                        <Route path="about" view=|cx| view! { cx, <About/> } />
                        <Route path="*" view=|cx| view! { cx, <PageNotFound/> } />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
