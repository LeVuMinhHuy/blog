use crate::routes::nav::{Nav, NavElements, NavProps};
use leptos::*;
use leptos_router::*;

#[component]
#[allow(non_snake_case)]
pub fn Blog(_cx: Scope) -> impl IntoView {
    // TODO: auto generate list of posts
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
                        <span class="post-date">"09-12-2022"</span>
                        <A href="1">"build this website using only rust-lang"</A>
                    </div>
                    <div class="post">
                        <span class="post-date">"08-12-2022"</span>
                        <A href="2">"from virtual dom to fine-grained reactivity, how leptos works"</A>
                    </div>
                    <div class="post">
                        <span class="post-date">"07-12-2022"</span>
                        <A href="2">"wasm with trunk"</A>
                    </div>

                </div>


                <Nav exclude={Some(NavElements::Blog)}/>
            </div>
        </div>
    }
}
