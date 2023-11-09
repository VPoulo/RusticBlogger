use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Posts {
    author: String,
    title: String,
    body: String,
    date: DateTime<Utc>,
}

impl Posts {
    // Creates a new struct of information.
    fn new(author: &str, title: &str, body: &str, date: DateTime<Utc>) -> Self {
        let author_string = author.to_string();
        let title_string = title.to_string();
        let body_string = body.to_string();

        Self {
            author: author_string,
            title: title_string,
            body: body_string,
            date,
        }
    }
}

fn main() {
    println!("Hello RusticBlogger, welcome!");

    create("Vaughn Poulo", "How to move to Berlin", "Just move!");

    read();
    delete("Whatever".to_string());
}

/// Adds a blog post to JSON file.
/// Returns true if successfully added.
fn create(author: &str, title: &str, body: &str) -> Result<bool, Box<dyn Error>> {
    // Create blog post in json format.
    let blog_post = Posts::new(author, title, body, Utc::now());

    // Open json File.
    let file_path = Path::new("../blog_posts_json.json");
    let file = File::open(file_path)?;
    //let reader = BufReader::new(file);

    // Parse file
    let all_posts: Vec<Posts> = serde_json::from_reader(file)?;

    // Add blog post to file.

    // Close file.

    // Convert the JSON value to a Rust String
    //let author_string: String = blog_post["Author"].as_str().unwrap_or("").to_string();

    //println!("Author: {}", author_string);

    Ok(true)
}

/// Returns all blog posts in the JSON file.
fn read() -> bool {
    return true;
}

/// Delets all blog posts from JSON file
/// that match blog title passed in. Returns
/// True if deleted successfully.
fn delete(_title: String) -> bool {
    return true;
}
