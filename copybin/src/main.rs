#[macro_use]
extern crate rocket;
extern crate dotenv;

mod db;
use dotenv::dotenv;
use std::env;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    for (key, value) in env::vars() {
        // println!("{}: {}", key, value);
    }

    db::qwq();

    rocket::build().mount("/", ![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
