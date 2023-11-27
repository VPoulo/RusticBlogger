use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::path::Path;
/// Holds all information for a
#[derive(Serialize, Deserialize, Debug)]
pub struct Posts {
    author: String,
    title: String,
    body: String,
    date: NaiveDate,
}

impl Posts {
    // Creates a new struct of information.
    pub fn new(author: &str, title: &str, body: &str, date: NaiveDate) -> Self {
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

/// Adds a blog post to JSON file. Returns true if successfully added.
pub fn create(author: &str, title: &str, body: &str) -> Result<bool, Box<dyn Error>> {
    let file_path = "../blog_posts_json.json";

    // Check if file exists, if not create it.
    if !Path::new(file_path).exists() {
        match File::create(file_path) {
            Ok(_) => println!("File created successfully"),
            Err(err) => println!("Error creating file: {}", err),
        }
    }

    // Convert to Pacific Standard Time.
    let current_time: DateTime<Utc> = Utc::now() - Duration::hours(8);

    // Convert to Date.
    let naive_date: NaiveDate = current_time.date_naive();

    // Create blog post in json format.
    let new_entry = Posts::new(author, title, body, naive_date);

    // Read file contents
    let file_contents =
        std::fs::read_to_string("../blog_posts_json.json").expect("Could not read from file");

    // Create vector to hold posts.
    let mut existing_posts: Vec<Posts>;

    // Check if json file has any existing posts. If so, load them into vector.
    if file_contents.is_empty() {
        existing_posts = Vec::new();
    } else {
        existing_posts = serde_json::from_str(&file_contents).unwrap();
    }

    // Parse file contents into vector of Posts.
    //let mut existing_posts: Vec<Posts> = serde_json::from_str(&file_contents).unwrap();

    // Add new blog post to vector.
    existing_posts.push(new_entry);

    // Create new JSON by serializing vector
    let updated_json = serde_json::to_string(&existing_posts)?;

    //Write over existing file with new JSON
    match std::fs::write(file_path, updated_json) {
        Ok(_) => return Ok(true),
        Err(err) => println!("Could not add post. Error {}", err),
    }

    Ok(true)
}

/// Returns all blog posts in the JSON file.
pub fn read() -> Vec<Posts> {
    // Read file contents
    let file_contents =
        std::fs::read_to_string("../blog_posts_json.json").expect("Could not read from file");

    // Parse file contents into vector of Posts.
    let existing_posts: Vec<Posts> = serde_json::from_str(&file_contents).unwrap();

    existing_posts
}

/// Delets all blog posts from JSON file that match blog title and
/// blog author. Returns true if deleted successfully.
pub fn delete(title: &str, author: &str) -> Result<bool, Box<dyn Error>> {
    let file = "../blog_posts_json.json";

    // Read file contents
    let file_contents =
        std::fs::read_to_string("../blog_posts_json.json").expect("Could not read from file");

    // Parse file contents into vector of Posts.
    let mut existing_posts: Vec<Posts> = serde_json::from_str(&file_contents).unwrap();

    // Find post in vector and delete it.
    existing_posts.retain(|post| {
        !(post.author.to_uppercase() == author.to_uppercase()
            && post.title.to_uppercase() == title.to_uppercase())
    });

    // Create new JSON by serializing vector
    let updated_json = serde_json::to_string(&existing_posts)?;

    //Write over existing file with new JSON
    match std::fs::write(file, updated_json) {
        Ok(_) => return Ok(true),
        Err(err) => println!("Could not add post. Error {}", err),
    }

    // Return option true
    Ok(true)
}

#[test]
fn test_create_delete() {
    let author = "Test Person";
    let title = "Title Test";
    let text = "Nothing";

    // Create post
    let result = create(author, title, text);
    assert_eq!(result.unwrap(), true);

    // Delete post
    let result: Result<bool, Box<dyn std::error::Error>> = delete(title, author);
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_read() {
    let author = "Test Person";
    let title = "Title Test";
    let text = "Nothing";

    // Create a post to make sure at least one exists.
    let create_result = create(author, title, text);
    assert_eq!(create_result.unwrap(), true);

    // Read posts
    let post_vec: Vec<Posts> = read();
    assert!(post_vec.len() > 0);

    // Delete post.
    let delete_result: Result<bool, Box<dyn std::error::Error>> = delete(title, author);
    assert_eq!(delete_result.unwrap(), true);
}
