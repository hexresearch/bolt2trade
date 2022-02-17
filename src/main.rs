#[macro_use] extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        parent : "base"
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", FileServer::from("static"))
      .mount("/", routes![index])
      .attach(Template::fairing())
}