mod generator;

fn main() {
    let _ = generator::convert_markdown_to_html();
    let _ = generator::update_blog_list_files();
    let _ = generator::update_blog_files();
}
