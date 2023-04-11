use crate::routes::nav::{Nav, NavElements, NavProps};
use leptos::*;
use leptos_router::*;

#[component]
#[allow(non_snake_case)]
pub fn Post(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);
    let _ = params().get("id").cloned().unwrap_or_default();

    //log!("{id}");

    // TODO:
    // 1. markdown to html
    // 2. get id and link it to the post

    view! {
        cx,
        <div>
            <div class="data">
                <div class="intro">
                    <p>"hi there"</p>
                    <p>"still struggling with leptos server"</p>
                    <p>"will finish markdown to html asap"</p>
                    <br/>
                </div>


                <Nav exclude={Some(NavElements::Blog)}/>
            </div>
        </div>
    }
}
