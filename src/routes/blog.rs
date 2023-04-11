use crate::routes::nav::{Nav, NavElements, NavProps};
use leptos::*;
use leptos_router::*;

#[component]
#[allow(non_snake_case)]
pub fn Blog(_cx: Scope) -> impl IntoView {
    view! {
        _cx,
        <div>
            <div class="data">
                <div class="title">
                    <p>"recently posts"</p>
                    <hr />
                </div>

                <div class="posts">
                    <div class="post-intro">
                        <span class="post-date">"dd-mm-yyyy"</span>
                        <span class="post-title">"title"</span>
                    </div>


                    <div class="post">
                        <span class="post-date">"10-04-2023"</span>
                        <A href="build-website-using-only-rust-lang">"build website using only rust lang"</A>
                    </div>

                </div>

                <Nav exclude={Some(NavElements::Blog)}/>
            </div>
        </div>
    }
}
