pub mod todotask;
pub mod users;

use std::{fmt::Display, sync::LazyLock};
use surrealdb::{engine::any::Any, opt::auth::Root, Surreal};

pub static DB:LazyLock<Surreal<Any>> = LazyLock::new(surrealdb::Surreal::init);

/// Connects the static singleton DB to the database via WS, localhost:8000
/// 
/// # Returns
/// `()` - Nothing
pub async fn connect() -> () {

    let _ = DB.connect("ws://127.0.0.1:8000")
        .await;
        // .map_err(|error| {
        //     match &error {
        //         surrealdb::Error::Api(api_error) => match api_error {
        //             surrealdb::error::Api::AlreadyConnected => {} // If the database is already connected, we dont have to do anything
        //             _ => panic!("Failed to connect to database: {}", error),
        //         },
        //         _ => panic!("Failed to connect to database: {}", error),
        //     }
        // });
    DB.use_ns("Dev").await.expect("Failed to use namespace 'Dev'");
    DB.use_db("Dev").await.expect("Failed to use database 'Dev'");
    DB.signin(Root {
        username: "root",
        password: "root",
    }).await.expect("Failed to login as root user");
}

#[allow(dead_code)]
/// Creates the tables and fields in the database
/// This function is used to create the tables and fields in the database
/// It is only used when setting up the database or during testing, not during production
/// 
/// # Returns
/// `()` - Nothing
pub async fn create_all() -> () {
    let mut response = DB.query("
    DEFINE TABLE User SCHEMAFULL;
    DEFINE FIELD username ON TABLE User TYPE string;
    DEFINE FIELD email ON TABLE User TYPE string;
    DEFINE FIELD password ON TABLE User TYPE string;
    DEFINE FIELD created_at ON TABLE User TYPE datetime DEFAULT time::now();
    DEFINE INDEX uniqueUsername ON TABLE User COLUMNS username UNIQUE;
    DEFINE INDEX uniqueEmail ON TABLE User COLUMNS email UNIQUE;

    DEFINE TABLE ToDoTask SCHEMAFULL; 
    DEFINE FIELD title ON TABLE ToDoTask TYPE string;
    DEFINE FIELD description ON TABLE ToDoTask TYPE option<string>;
    DEFINE FIELD owner ON TABLE ToDoTask TYPE record<User>;
    DEFINE FIELD completed_at ON TABLE ToDoTask TYPE option<datetime>;
    DEFINE FIELD created_at ON TABLE ToDoTask TYPE datetime DEFAULT time::now();

    ")
    .await
    .expect("Failed to create table ToDoTask and fields"); // Its okay for this function to panic as it is only used when setting up the database or during testing, not during production

    let result = response.take_errors();
    if result.is_empty() {
        println!("Table and fields created successfully.");
    } else {
        panic!("Failed to create table and fields: {:?}", result);
    }
}

#[allow(dead_code)]
/// Clear all test data from the database
/// 
/// # Returns
/// `()` - Nothing
pub async fn clear_all_test() -> () {
    let sql = "
    DELETE User WHERE username CONTAINS \"TEST\";
    DELETE ToDoTask WHERE title CONTAINS \"TEST\";";

    let mut response = DB.query(sql)
        .await
        .expect("Failed to clear test data");

    let result: Option<String> = response.take(0).expect("Failed to clear test data: ");

    println!("Cleared test data: {:?}", result);
}

#[derive(Debug, Clone)]
/// Error type returned when creating records in the database
/// 
/// # Variants
/// * `AlreadyExists` - The record already exists in the database
/// * `BadData` - The data provided is invalid
/// * `Other` - Any other error that may occur
pub enum DBCreateError {
    #[allow(dead_code)]
    AlreadyExists(String),
    BadData(String),
    Other(String)
}

#[derive(Debug, Clone)]
/// Error type returned when editing records in the database
/// 
/// # Variants
/// * `NotFound` - The record was not found in the database
/// * `BadData` - The data provided is invalid
/// * `Other` - Any other error that may occur
pub enum DBEditError {
    NotFound(String),
    BadData(String),
    Other(String)
}

#[derive(Debug, Clone)]
/// Error type returned when reading records from the database
/// 
/// # Variants
/// * `NotFound` - The record was not found in the database
/// * `Other` - Any other error that may occur
pub enum DBReadError {
    NotFound(String),
    Other(String),
}

impl Display for DBCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBCreateError::AlreadyExists(msg) => write!(f, "Already exists error: {}", msg),
            DBCreateError::BadData(msg) => write!(f, "Bad data error: {}", msg),
            DBCreateError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Display for DBEditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBEditError::NotFound(msg) => write!(f, "Not found error: {}", msg),
            DBEditError::BadData(msg) => write!(f, "Bad data error: {}", msg),
            DBEditError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl Display for DBReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBReadError::NotFound(msg) => write!(f, "Not found error: {}", msg),
            DBReadError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}