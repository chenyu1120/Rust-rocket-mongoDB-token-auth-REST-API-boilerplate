use bcrypt::hash;
use chrono::{NaiveTime, Utc};
use mongodb::{bson, bson::oid::ObjectId, options::ClientOptions, Client, Database};
use rocket::serde::json::Json;
use rocket::{fairing::AdHoc, futures::TryStreamExt};
use std::collections::HashSet;

use crate::database::url_mongo_db::DB;

pub struct MongoDB {
    pub(crate) database: Database,
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Connect to MongoDB cluster", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(MongoDB::new(database)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let client_options = ClientOptions::parse(DB).await?;
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(bson::doc! {"ping": 1}, None)
        .await?;

    println!("connected to DB");

    Ok(client.database("authorization"))
}