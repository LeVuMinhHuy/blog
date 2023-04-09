use crate::routes::nav::{Nav, NavElements, NavProps};
use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn Home(_cx: Scope) -> impl IntoView {
    view! {
        _cx,
        <div>
            <div class="data">
                <div class="intro">
                    <p>"hi there"</p>
                    <br/>

                    <p>"i'm ted"</p>
                    <p>"i do code. using js for work and working with rust for fun"</p>
                    <p>"sometimes i also do write, about tech and thoughts, here"</p>
                </div>

                <Nav exclude={Some(NavElements::Home)}/>
            </div>
        </div>
    }
}
