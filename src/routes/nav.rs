use leptos::*;
use leptos_router::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NavElements {
    Blog,
    About,
    Home,
}

#[component]
#[allow(non_snake_case)]
pub fn Nav(cx: Scope, exclude: Option<NavElements>) -> impl IntoView {
    let exclude = exclude.unwrap();
    let nav_elements = vec![
        (NavElements::Blog, "/blog", "# blog"),
        (NavElements::About, "/about", "# more about me"),
        (NavElements::Home, "/", "# home"),
    ];

    let nav_elements = nav_elements
        .into_iter()
        .filter(|(e, _, _)| *e != exclude)
        .map(|(e, href, text)| {
            let exact = e == NavElements::Home;
            view! {
                cx,
                <A exact=exact href=href><p>{text}</p></A>
            }
        })
        .collect::<Vec<_>>();

    view! {
        cx,
        <nav class="nav">
           <div class="nav-text">
            {nav_elements}
           </div>
        </nav>
    }
}
