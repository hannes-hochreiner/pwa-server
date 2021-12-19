use std::env;

use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/config", routes![index])
        .mount("/", FileServer::from(env::var("ROOT_FS_PATH").unwrap()))
}
