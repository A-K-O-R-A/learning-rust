#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    for (key, value) in env::vars() {
        // println!("{}: {}", key, value);
    }

    rocket::build().mount("/", ![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
