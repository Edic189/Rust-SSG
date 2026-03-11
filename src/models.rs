use serde::Serialize;

// Represents a blog post's metadata for the template engine
#[derive(Serialize)]
pub struct BlogPost {
    pub title: String,
    pub slug: String,
}