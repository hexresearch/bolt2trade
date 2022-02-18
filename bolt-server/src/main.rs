#[macro_use] extern crate rocket;
use std::error::Error;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};
use bolt_db::db::*;

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        parent : "base"
    })
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//       .mount("/", FileServer::from("static"))
//       .mount("/", routes![index])
//       .attach(Template::fairing())
// }

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>>{
    tokio::spawn( async move {
        if let Ok(db) = initdb().await {
            db.insert_val(1, "1").await.unwrap();
        }
    });
    rocket::build()
      .mount("/", FileServer::from("static"))
      .mount("/", routes![index])
      .attach(Template::fairing())
      .launch().await?;
    Ok(())
}