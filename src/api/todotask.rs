use rocket::{post, serde::json::Json};
use crate::database::todotask::create_task;
use crate::model::todotask::ToDoTask;
use super::Response;

#[post("/tasks", data = "<input_task>")]
pub async fn create_task_handler(
    input_task: Json<ToDoTask>
) -> Response<Json<ToDoTask>> {
    let input_task = input_task.into_inner(); // Deserialise the input from JSON
    println!("Input task: {:?}", input_task);

    // Option<String> -> Option<&str>
    let title = input_task.title.as_deref();
    let description = input_task.description.as_deref();
    let completed_at = input_task.completed_at.as_deref();

    // Check if the title is empty
    if title.is_none() {
        return Response::BadRequest("Title is required".to_string());
    }
    let title = title.unwrap();

    // Create the task 
    let created_task = create_task(title, description, completed_at, None).await;
    println!("Created task: {:?}", created_task);

    // Check if there was an error
    if created_task.is_err() {
        let err = created_task.unwrap_err();
        return match err {
            crate::database::DBCreateError::Permissions(_) => Response::Forbidden("You do not have permission to create this task".to_string()),
            crate::database::DBCreateError::AlreadyExists(_) => Response::BadRequest("This task already exists".to_string()),
            crate::database::DBCreateError::BadData(_) => Response::BadRequest("The data provided is invalid".to_string()),
            crate::database::DBCreateError::Other(_) => {
                dbg!("Unhandled/Unkown error creating task: {:?}", err);
                Response::InternalServerError("There was an unkown error".to_string())
            }
        }
    }
    let task = created_task.unwrap();
    println!("Task created: {:?}", task);

    // Return the response
    Response::Created(Json(task))
}