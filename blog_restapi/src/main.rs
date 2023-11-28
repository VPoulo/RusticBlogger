#[macro_use]
extern crate rocket;
extern crate blog_restapi;

use rocket::form::Form;
use rocket::form::FromForm;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

// From to use with new post form to grab fields.
#[derive(FromForm)]
struct BlogPost {
    author: String,
    title: String,
    body: String,
}

/// Load landing page with all existing blog posts.
#[get("/")]
fn index() -> Template {
    let posts: Vec<blog_restapi::Posts> = blog_restapi::read();
    let context = context! { posts: posts };
    Template::render("index", context)
}

// Load new post form to add a new post to the blog.
#[get("/post")]
fn new_post_page() -> Template {
    Template::render("post", context! {})
}

/// Process a new post (form) by adding it to the json file.
#[post("/post", data = "<post>")]
fn new_post(post: Form<BlogPost>) -> Redirect {
    // Grab data from form.
    let author = post.author.clone();
    let title = post.title.clone();
    let body = post.body.clone();

    // If new blog post successfully created, then re-direct to index page after.
    if let Ok(true) = blog_restapi::create(&author, &title, &body) {
        rocket::response::Redirect::to("/")
    } else {
        rocket::response::Redirect::to("/error")
    }
}

/// Delete a post and redirect back to landing page.
#[delete("/delete/<title>/<author>")]
fn delete(title: String, author: String) -> Redirect {
    if let Ok(true) = blog_restapi::delete(&title, &author) {
        // If new blog post successfully deleted, then re-direct to index page after.
        rocket::response::Redirect::to("/error")
    } else {
        // Return error page if error.
        rocket::response::Redirect::to("/error")
    }
}

/// General error
#[get("/error")]
fn error() -> Template {
    Template::render("error", context! {})
}

/// Catch all internal server errors.
#[catch(500)]
fn internal_server_error() -> Template {
    Template::render("error", context! {})
}

/// Lanuch the app.
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, delete, new_post_page, new_post, error])
        .attach(Template::fairing())
        .register("/", catchers![internal_server_error])
}
