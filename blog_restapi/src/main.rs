#[macro_use]
extern crate rocket;
extern crate blog_restapi;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    let posts: Vec<blog_restapi::Posts> = blog_restapi::read();
    let context = context! { posts: posts };
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
