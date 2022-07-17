#[macro_use]
extern crate rocket;
extern crate dotenv;

use db::Bin;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;
use mongodb::Collection;
use rocket::fs::FileServer;
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
        .mount("/", FileServer::from("./static"))
        .mount("/", routes![find_id, create_test])
}

#[get("/api/find_id/<id>")]
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

#[post("/api/create", data = "<bin_data>")]
async fn create_test(
    db: Connection<Bins>,
    bin_data: Json<db::UploadedBin>,
) -> Option<Json<InsertOneResult>> {
    let collection: Collection<Bin> = db.database("copybin").collection("bins");

    // Insert some documents into the "mydb.books" collection.
    let random_id = "Abs".to_string();
    let bin = Bin {
        id: random_id,
        code: bin_data.0.code,
        language: bin_data.0.language,
    };
    let res = collection.insert_one(bin, None).await;

    match res {
        Ok(r) => Some(Json(r)), //result.inserted_id.as_str().unwrap().clone()
        Err(error) => {
            println!("{}", error);
            None
        }
    }
}
