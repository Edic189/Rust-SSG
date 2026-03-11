# Rust Custom Static Site Generator

A custom, lightweight Static Site Generator (SSG) built entirely in Rust. It converts Markdown files into static HTML pages, designed for simplicity, speed, and easy automation with Markdown-based note-taking tools like Obsidian.

## Project Structure

The repository is organized into four distinct directories:

* **`content/`**: Contains the raw Markdown (`.md`) files. The `blog/` subdirectory is processed dynamically, while root files like `index.md` are handled manually.
* **`src/`**: Contains the core Rust engine responsible for site generation.
* **`static/`**: Houses all static assets, including CSS stylesheets, images, and favicons.
* **`templates/`**: Contains the HTML layout structures, such as `base.html` and `blog_list.html`.

## Core Engine (`src/`)

The generation logic is separated into four key Rust modules:

* **`main.rs`**: The main execution point. It creates the `public/` output directory, copies static assets from the `static/` folder, initializes the template engine, and triggers the build process for both the standalone pages and the dynamic blog section.
* **`blog.rs`**: Handles the blog content pipeline. It iterates through the `content/blog/` directory, extracts the first `# ` heading to use as the post title, generates individual HTML files, and compiles the main blog archive index.
* **`models.rs`**: Defines the data structures. It contains the `BlogPost` struct (holding the title and slug), utilizing Serde to pass serialized data to the template engine.
* **`utils.rs`**: Contains essential helper functions. It includes a recursive directory copy function for static assets and a universal `build_page` function that parses Markdown into HTML and injects it into the templates.

## Templating

The project uses a template engine to maintain a consistent layout across all generated pages. The `base.html` template acts as the main wrapper, accepting raw parsed HTML via a `{{ content | safe }}` tag and dynamically resolving relative paths for CSS and images using a `{{ root_path }}` variable.

## How to Run

1. **Prerequisites**: Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.
2. **Build and Generate**: Run the following command in the root of the project to compile the Rust code and generate the site:
   ```bash
   cargo run
   ```
   This will process everything and output the final, ready-to-deploy HTML files into the `public/` directory.