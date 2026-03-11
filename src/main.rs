mod models;
mod utils;
mod blog;

use minijinja::Environment;
use std::fs;
use std::path::Path;
use utils::{build_page, copy_dir_all};

fn main() {
    println!("--- Starting ---");
    // Ensure the output directory for blog posts exists
    let _ = fs::create_dir_all("public/blog");

    // Copy static image assets to the public directory
    if Path::new("static/blog").exists() {
        copy_dir_all("static/blog", "public/slike/blog").expect("Failed to copy blog images!");
        println!("Copied images to public/slike/blog/");
    }
    
    // Copy favicon assets
    if Path::new("static/favicon").exists() {
        copy_dir_all("static/favicon", "public/slike/favicon").expect("Failed to copy favicons!");
        println!("Copied favicons to public/slike/favicon/");
    }

    // Copy the main stylesheet
    if Path::new("static/style.css").exists() {
        fs::copy("static/style.css", "public/style.css").unwrap();
    }

    // Initialize the Minijinja template environment
    let mut env = Environment::new();
    
    // Load the base layout template
    let template_str = fs::read_to_string("templates/base.html").unwrap();
    env.add_template("base", &template_str).unwrap();

    // Load the template for the blog post list
    let blog_list_str = fs::read_to_string("templates/blog_list.html").unwrap();
    env.add_template("blog_list", &blog_list_str).unwrap();

    // Build standalone static pages (e.g., Home, About)
    println!("--- Main pages ---");
    let manual_pages = vec![
        ("content/index.md", "public/index.html"),
        ("content/about.md", "public/about.html"),
    ];
    
    for (in_path, out_path) in manual_pages {
        if Path::new(in_path).exists() {
            build_page(&env, in_path, out_path, "");
        }
    }

    // Build the blog section (individual posts and the index page)
    blog::build(&env);
}