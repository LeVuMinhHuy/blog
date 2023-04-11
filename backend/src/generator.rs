use chrono::{DateTime, Local};
use pulldown_cmark::{html, Parser};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use walkdir::WalkDir;

pub fn convert_markdown_to_html() -> io::Result<()> {
    let md_folder = Path::new("../posts");
    let html_folder = Path::new("./posts");

    if !html_folder.exists() {
        fs::create_dir_all(html_folder)?;
    }

    for entry in WalkDir::new(md_folder) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let post_id = html_folder.join(path.strip_prefix(md_folder).unwrap());
            let post_id = post_id.with_extension("html");

            if let Ok(md_modified) = get_modified_time(path) {
                if let Ok(html_modified) = get_modified_time(&post_id) {
                    if md_modified <= html_modified {
                        continue;
                    }
                }
            }

            let mut md_file = File::open(path)?;
            let mut md_content = String::new();
            md_file.read_to_string(&mut md_content)?;

            let html_content = markdown_to_html(&md_content);

            let mut html_file = File::create(&post_id)?;
            html_file.write_all(html_content.as_bytes())?;
        }
    }

    Ok(())
}

fn get_modified_time(path: &Path) -> io::Result<DateTime<Local>> {
    let metadata = path.metadata()?;
    let modified = metadata.modified()?;

    Ok(DateTime::from(modified))
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_output, parser);
    html_output
}

fn generate_list_posts() -> String {
    let html_folder = Path::new("./posts");
    let mut posts = Vec::new();
    for entry in WalkDir::new(html_folder) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
            let post_id = path.file_stem().unwrap().to_str().unwrap();
            let post_title = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .split('-')
                .collect::<Vec<&str>>()
                .join(" ");
            let post_date = get_modified_time(path)
                .unwrap()
                .format("%d-%m-%Y")
                .to_string();

            posts.push(format!(
                r#"
                            <div class="post">
                                <span class="post-date">"{}"</span>
                                <A href="{}">"{}"</A>
                            </div>
                "#,
                post_date, post_id, post_title
            ));
        }
    }

    format!(
        r#"
        use crate::routes::nav::{{Nav, NavElements, NavProps}};
        use leptos::*;
        use leptos_router::*;
        
        #[component]
        #[allow(non_snake_case)]
        pub fn Blog(_cx: Scope) -> impl IntoView {{
            view! {{
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
        
                            {}
                        </div>
        
                        <Nav exclude={{Some(NavElements::Blog)}}/>
                    </div>
                </div>
            }}
        }}
        "#,
        posts.join("")
    )
}

pub fn update_blog_list_files() -> io::Result<()> {
    let blog_file = Path::new("../src/routes/blog.rs");
    let mut blog_file = File::create(blog_file)?;
    blog_file.write_all(generate_list_posts().as_bytes())?;
    Ok(())
}

fn generate_post() -> Vec<Vec<String>> {
    let html_folder = Path::new("./posts");
    let html_ui_folder = Path::new("../src/posts");

    let mut posts: Vec<Vec<String>> = Vec::new();
    if !html_ui_folder.exists() {
        fs::create_dir_all(html_ui_folder).unwrap();
    }

    for entry in WalkDir::new(html_folder) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "html") {
            let post_id = html_ui_folder.join(path.strip_prefix(html_folder).unwrap());
            let post_id = post_id.with_extension("html");

            if let Ok(html_modified) = get_modified_time(path) {
                if let Ok(html_ui_modified) = get_modified_time(&post_id) {
                    if html_ui_modified <= html_modified {
                        continue;
                    }
                }
            }

            let mut html_file = File::open(path).unwrap();
            let mut html_content = String::new();
            html_file.read_to_string(&mut html_content).unwrap();
            let data = format!(
                r#"
                <div>
                    <div class="data">
                        <div class="intro">
                            {}
                        </div>
                    </div>
                </div>
                "#,
                html_content
            );

            posts.push(vec![post_id.to_str().unwrap().to_string(), data]);
        }
    }
    posts
}

pub fn update_blog_files() -> io::Result<()> {
    let blog_file = Path::new("../src/posts");
    if !blog_file.exists() {
        fs::create_dir_all(blog_file).unwrap();
    };
    for post in generate_post() {
        let post_file = Path::new(&post[0]);
        let mut post_file = File::create(post_file)?;
        post_file.write_all(post[1].as_bytes())?;
    }

    Ok(())
}
