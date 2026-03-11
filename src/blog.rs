use crate::models::BlogPost;
use crate::utils::build_page;
use minijinja::{context, Environment};
use std::fs;

pub fn build(env: &Environment) {
    println!("--- Processing blog ---");
    let mut posts = Vec::new();

    // Read all files in the blog content directory
    if let Ok(entries) = fs::read_dir("content/blog") {
        for entry in entries.flatten() {
            let path = entry.path();
            
            // Process only Markdown (.md) files
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let out_path = format!("public/blog/{}.html", file_stem);
                
                let md_content = fs::read_to_string(&path).unwrap_or_default();
                
                // Extract the first H1 heading (# ) to use as the blog post title
                let mut title = String::from(file_stem);
                for line in md_content.lines() {
                    if line.starts_with("# ") {
                        title = line.trim_start_matches("# ").to_string();
                        break;
                    }
                }

                // Generate the individual HTML page for the blog post
                build_page(env, path.to_str().unwrap(), &out_path, "../");

                // Save post metadata to generate the blog index later
                posts.push(BlogPost {
                    title,
                    slug: file_stem.to_string(),
                });
            }
        }
    }

    // Render the list of blog posts using the blog_list template
    let blog_list_tmpl = env.get_template("blog_list").unwrap();
    let blog_content_html = blog_list_tmpl.render(context!(posts => posts)).unwrap();

    // Wrap the rendered blog list inside the base layout template
    let base_tmpl = env.get_template("base").unwrap();
    let final_blog_index = base_tmpl.render(context!(content => blog_content_html, root_path => "")).unwrap();
    
    // Write the final blog index to the public directory
    fs::write("public/blog.html", final_blog_index).unwrap();
    println!("Done!");
}