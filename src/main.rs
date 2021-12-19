use std::env;

use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            FileServer::from(env::var("ROOT_DIR").unwrap()).rank(10),
        )
        .mount(
            "/config",
            FileServer::from(env::var("CONFIG_DIR").unwrap()).rank(0),
        )
}
