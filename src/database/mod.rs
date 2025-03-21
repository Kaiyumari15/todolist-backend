use std::sync::LazyLock;

use surrealdb::{engine::{any::Any, remote::ws::{Client, Ws}}, Surreal};

pub static DB:LazyLock<Surreal<Any>> = LazyLock::new(surrealdb::Surreal::init);

/// Connects the static singleton DB to the database via WS, localhost:8000
pub async fn connect() -> () {
    DB.connect("127.0.0.1")
        .await
        .expect("Failed to connect to SurrealDB");
    DB.use_ns("Dev").await.expect("Failed to use namespace 'Dev'");
    DB.use_db("Dev").await.expect("Failed to use database 'Dev'");
}

pub async fn create_all() -> () {
    let mut response = DB.query("
    DEFINE TABLE ToDoTask SCHEMAFULL;
    DEFINE FIELD title ON TABLE ToDoTask TYPE string;
    DEFINE FIELD description ON TABLE ToDoTask TYPE option<string>;
    DEFINE FIELD completed_at ON TABLE ToDoTask TYPE option<datetime>;
    DEFINE FIELD created_at ON TABLE ToDoTask TYPE datetime VALUE time::now();
    ")
    .await
    .expect("Failed to create table ToDoTask and fields");

    let result = response.take_errors().is_empty();
    if result {
        println!("Table and fields created successfully.");
    } else {
        println!("Failed to create table and fields: {:?}", response.take_errors());
    }
}