#[macro_use]
extern crate rocket;
extern crate dotenv;

use rocket_db_pools::mongodb;
use rocket_db_pools::{Connection, Database};

mod db;
use dotenv::dotenv;

#[derive(Database)]
#[database("bins")]
struct Bins(mongodb::Client);

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let (_client, _db) = db::init().await.expect("MongoDB");

    rocket::build()
        .attach(Bins::init())
        .mount("/", routes![index, owo])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/owo")]
fn owo(mut db: Connection<Bins>) -> &'static str {
    let db = db.database("copybin");

    "Hello, world!"
}
