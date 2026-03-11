use minijinja::{context, Environment};
use pulldown_cmark::{html, Parser};
use std::fs;
use std::io;
use std::path::Path;

// Recursively copies a directory and all its contents to a destination
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

// Converts a Markdown file to HTML, wraps it in the base template, and saves it
pub fn build_page(env: &Environment, md_path: &str, html_path: &str, root_path: &str) {
    let md_content = fs::read_to_string(md_path)
        .unwrap_or_else(|_| format!("Warning: Missing content in {}", md_path));
    
    // Parse Markdown content into raw HTML
    let parser = Parser::new(&md_content);
    let mut html_body = String::new();
    html::push_html(&mut html_body, parser);

    // Inject the raw HTML into the base layout template
    let tmpl = env.get_template("base").unwrap();
    let final_html = tmpl.render(context!(content => html_body, root_path => root_path)).unwrap();

    // Write the generated HTML file to the disk
    fs::write(html_path, final_html).unwrap();
    println!("Created {}", html_path);
}