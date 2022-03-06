#[macro_use] extern crate rocket;
use std::error::Error;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, context};
use bolt_db::db::*;
use tokio::sync::Mutex;
use std::sync::Arc;
use rocket::State;

#[get("/")]
async fn index(db: &State<Arc<Mutex<DbConn>>>) -> Template {
    Template::render("index", context!{
        parent : "base"
    })
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let db = initdb().await.expect("DB failed to initialize");
    let dbarc = Arc::new(Mutex::new(db));
    tokio::spawn( {
        let dbarc = dbarc.clone();
        async move {
            let db = dbarc.lock().await;
            let key = "lol".to_string();
            db.insert_user(key.clone(), Some("anon".to_string())).await.expect("Failed to insert user");
            let ou = db.get_user(key.clone()).await.expect("Failed to get user");
            println!("{:?}", ou);
            db.delete_user(key.clone()).await.expect("Failed to delete user");
            let ou = db.get_user(key.clone()).await.expect("Failed to get user");
            println!("{:?}", ou);
        }
    });
    rocket::build()
      .mount("/", FileServer::from("static"))
      .mount("/", routes![index])
      .manage(dbarc)
      .attach(Template::fairing())
      .launch().await?;
    Ok(())
}