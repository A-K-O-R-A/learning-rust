use mongodb::{Client, Database};
use std::env;
use std::error::Error;

pub async fn init() -> Result<(Client, Database), Box<dyn Error>> {
    // Parse a connection string into an options struct.
    //let client_options = ClientOptions::parse(env::var("MONGO_URI").expect("MONG_URI")).await?;

    // Manually set an option.
    //client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    //let client = Client::with_options(client_options)?;
    let client = Client::with_uri_str(env::var("MONGO_URI").expect("MONG_URI")).await?;

    // List the names of the databases in that deployment.
    //for db_name in client.list_database_names(None, None).await? {
    //    println!("{}", db_name);
    //}

    // Get a handle to a database.
    let db = client.database("copybin");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }

    Ok((client, db))
}
