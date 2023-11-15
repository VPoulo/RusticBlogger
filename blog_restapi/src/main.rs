#[macro_use]
extern crate rocket;
extern crate blog_restapi;
use rocket_dyn_templates::Template;

// fn main() {
//     rocket();
// }

#[get("/")]
fn index() -> Template {
    let posts: Vec<blog_restapi::Posts> = blog_restapi::read();
    // let mut context = tera::Context::new();
    // context.insert("posts", &posts);
    Template::render("index", &posts)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
