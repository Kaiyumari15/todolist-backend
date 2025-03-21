use std::sync::LazyLock;

use surrealdb::{engine::{any::Any, remote::ws::{Client, Ws}}, Surreal};

pub static DB:LazyLock<Surreal<Any>> = LazyLock::new(surrealdb::Surreal::init);

pub async fn connect() -> () {
    DB.connect("127.0.0.1")
        .await
        .expect("Failed to connect to SurrealDB");
    DB.use_ns("Dev").await.expect("Failed to use namespace 'Dev'");
    DB.use_db("Dev").await.expect("Failed to use database 'Dev'");
}