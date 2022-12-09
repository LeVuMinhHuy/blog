use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod routes;
use routes::about::*;
use routes::blog::*;
use routes::home::*;

// TODO: handle responsive

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());
    view! {
        cx,
        <div id="root">
            <Router>
                <main>
                    <Routes>
                        <Route path="" element=move |_cx| view! { cx, <Home/> } />
                        <Route path="blog" element=move |_cx| view! { cx, <Blog/> } />
                        <Route path="about" element=move |_cx| view! { cx, <About/> } />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
