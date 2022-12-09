use leptos::*;
use leptos_router::*;

#[component]
#[allow(non_snake_case)]
pub fn Post(cx: Scope) -> Element {
    let _params = use_params_map(cx);
    //let post_data: _ = create_resource(
    //    cx,
    //    move || params().get("id").cloned().unwrap_or_default(),
    //    generate_post,
    //);

    view! {
        cx,
        <div>
            <div class="data">
                <div class="intro">
                    <p>"hi there"</p>
                    <br/>

                </div>

                <nav class="nav">
                    <div class="nav-text">
                       <A exact=true href="/blog"><p>"# blog"</p></A>
                       <A href="/about"><p>"# more about me"</p></A>
                    </div>
                </nav>
            </div>
        </div>
    }
}

//fn generate_post(post_name: String) {
//    let paths = fs::read_dir("../posts").unwrap();
//
//    for path in paths {
//        println!("Name: {}", path.unwrap().path().display())
//
//        // let contents = fs::read_to_string("../posts").expect("Should have been able to read the file");
//        // println!("With text:\n{contents}");
//    }
//}
