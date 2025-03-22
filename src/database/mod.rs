mod todotask;

use std::{fmt::Display, sync::LazyLock};
use surrealdb::{engine::any::Any, Surreal};

pub static DB:LazyLock<Surreal<Any>> = LazyLock::new(surrealdb::Surreal::init);

/// Connects the static singleton DB to the database via WS, localhost:8000
pub async fn connect() -> () {
    DB.connect("ws://127.0.0.1:8000")
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
    .expect("Failed to create table ToDoTask and fields"); // Its okay for this function to panic as it is only used when setting up the database or during testing, not during production

    let result = response.take_errors().is_empty();
    if result {
        println!("Table and fields created successfully.");
    } else {
        println!("Failed to create table and fields: {:?}", response.take_errors());
    }
}

#[derive(Debug, Clone)]
/// The error type which will be returned when creating a task in the database
pub enum DBCreateError {
    Permissions(String),
    AlreadyExists(String),
    BadData(String),
    Other(String)
}

#[derive(Debug, Clone)]
/// The error type which will be returned when editing a task in the database
pub enum DBEditError {
    Permissions(String),
    NotFound(String),
    BadData(String),
    Other(String)
}

#[derive(Debug, Clone)]
/// The error type which will be returned when reading a task in the database
pub enum DBReadError {
    Permissions(String),
    NotFound(String),
    Other(String),
}

impl Display for DBCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBCreateError::Permissions(msg) => write!(f, "Permissions error: {}", msg),
            DBCreateError::AlreadyExists(msg) => write!(f, "Already exists error: {}", msg),
            DBCreateError::BadData(msg) => write!(f, "Bad data error: {}", msg),
            DBCreateError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Display for DBEditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBEditError::Permissions(msg) => write!(f, "Permissions error: {}", msg),
            DBEditError::NotFound(msg) => write!(f, "Not found error: {}", msg),
            DBEditError::BadData(msg) => write!(f, "Bad data error: {}", msg),
            DBEditError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Display for DBReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBReadError::Permissions(msg) => write!(f, "Permissions error: {}", msg),
            DBReadError::NotFound(msg) => write!(f, "Not found error: {}", msg),
            DBReadError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}