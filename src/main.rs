use api::{todotask::{create_task_handler, delete_task_handler, get_task_handler, update_task_handler, get_tasks_by_user_handler}, user::{create_user_handler, sign_in_user_handler}};
use rocket::routes;

mod api;
mod database;
mod model;
mod tests;

#[rocket::main]
async fn main() {
    println!("Hello, world!");
    let _ = database::connect().await;
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                create_user_handler,
                sign_in_user_handler,
                create_task_handler,
                get_task_handler,
                get_tasks_by_user_handler,
                update_task_handler,
                delete_task_handler
            ],
        )
        .launch()
        .await
        .expect("Error launching rocket instance");
}
