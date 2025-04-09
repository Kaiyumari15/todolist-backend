mod todotasks;
mod users;

use rocket::{build, routes};

pub fn rocket_test_launch() -> rocket::Rocket<rocket::Build> {
    build()
        .mount("/tasks", routes![
            crate::api::todotask::create_task_handler,
            crate::api::todotask::update_task_handler,
            crate::api::todotask::delete_task_handler,
        ])
        .mount("/users", routes![
            crate::api::user::create_user_handler,
            crate::api::user::sign_in_user_handler,
        ])
}