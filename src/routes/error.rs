use crate::routes::nav::{Nav, NavProps};
use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn PageNotFound(_cx: Scope) -> impl IntoView {
    view! {
        _cx,
        <div>
            <div class="data">
                <p>"hey it's 404"</p>
                <br/>
                <br/>

                <Nav exclude={None}/>
            </div>
        </div>
    }
}
