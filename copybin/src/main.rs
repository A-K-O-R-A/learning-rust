#[macro_use]
extern crate rocket;
extern crate dotenv;

use db::Bin;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use rocket::serde::json::Json;
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
        .mount("/", routes![index, find_id, uwu, create_test])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/find_id/<id>")]
async fn find_id(db: Connection<Bins>, id: &str) -> Option<Json<Bin>> {
    let collection = db.database("copybin").collection::<Bin>("bins");
    let res = collection.find_one(doc! { "id": id}, None).await;

    match res {
        Ok(result) => match result {
            Some(r) => Some(Json(r)),
            None => None,
        },
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

#[put("/create", data = "<bin_data>")]
async fn create_test(db: Connection<Bins>, bin_data: Json<Bin>) -> Option<Json<InsertOneResult>> {
    let collection: Collection<Bin> = db.database("copybin").collection("bins");

    // Insert some documents into the "mydb.books" collection.
    let res = collection.insert_one(bin_data.0, None).await;

    match res {
        Ok(r) => Some(Json(r)), //result.inserted_id.as_str().unwrap().clone()
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}

#[post("/uwu")]
async fn uwu(_db: Connection<Bins>) -> &'static str {
    //let coll: Collection<Bin> = db.database("copybin").collection("bins");
    //let res = coll.distinct("id", doc! { "id":"a"}, None).await;
    "Hello, world!"
}
