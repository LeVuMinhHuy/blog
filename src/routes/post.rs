use crate::routes::nav::{Nav, NavElements, NavProps};
use leptos::*;
use leptos_router::*;

#[component]
#[allow(non_snake_case)]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let _ = params.with(|params| params.get("id").cloned()).unwrap();

    view! {
        cx,
        <div>
            <div class="data">
                <div class="intro">
                    <p>"hi there"</p>
                    <br/>
                </div>

                <Nav exclude={Some(NavElements::Blog)}/>
            </div>
        </div>
    }
}
