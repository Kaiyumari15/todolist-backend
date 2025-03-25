use api::todotask::create_task_handler;
use rocket::routes;

mod api;
mod database;
mod model;
mod tests;

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    let _ = database::connect().await;
    let rocket = rocket::build()
        .mount("/", routes![create_task_handler])
        .launch()
        .await
        .expect("Error launching rocket instance");
}
