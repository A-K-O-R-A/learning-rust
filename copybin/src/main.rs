#[macro_use]
extern crate rocket;
extern crate dotenv;

use db::Bin;
use mongodb::bson::doc;
use mongodb::Collection;
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
        .mount("/", routes![index, owo, uwu])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/owo")]
async fn owo(mut db: Connection<Bins>) -> &'static str {
    let coll: Collection<Bin> = db.database("copybin").collection("bins");
    let res = coll.distinct("id", doc! { "id":"a"}, None).await;
    "Hello, world!"
}

#[post("/uwu")]
async fn uwu(mut db: Connection<Bins>) -> &'static str {
    let coll: Collection<Bin> = db.database("copybin").collection("bins");
    let res = coll.distinct("id", doc! { "id":"a"}, None).await;
    "Hello, world!"
}
