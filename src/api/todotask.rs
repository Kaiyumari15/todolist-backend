use chrono::format;
use rocket::{post, patch, delete, serde::json::Json};
use crate::database::todotask::{check_is_owner, create_task, delete_task_by_id, edit_task_by_id};
use crate::model::todotask::ToDoTask;
use super::Response;

#[post("/tasks", data = "<input_task>")]
pub async fn create_task_handler(
    input_task: Json<ToDoTask>
) -> Response<Json<ToDoTask>> {
    let input_task = input_task.into_inner(); // Deserialise the input from JSON

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

    // Check if there was an error
    if created_task.is_err() {
        let err = created_task.unwrap_err();
        return match err {
            crate::database::DBCreateError::AlreadyExists(_) => Response::BadRequest("This task already exists".to_string()),
            crate::database::DBCreateError::BadData(_) => Response::BadRequest("The data provided is invalid".to_string()),
            crate::database::DBCreateError::Other(_) => {
                dbg!("Unhandled/Unkown error creating task: {:?}", err);
                Response::InternalServerError("There was an unkown error".to_string())
            }
        }
    }
    let task = created_task.unwrap();

    // Return the response
    Response::Created(Json(task))
}

#[patch("/tasks/<task_id>", data="<update_task>")]
pub async fn update_task_handler(task_id: &str, update_task: Json<ToDoTask>) -> super::Response<Json<ToDoTask>> {

    // Deserialise the input from JSON
    let update_task = update_task.into_inner();

    // Option<String> -> Option<&str>
    let title = update_task.title.as_deref();
    let description = update_task.description.as_deref();
    let completed_at = update_task.completed_at.as_deref();
    let owner = update_task.owner.and_then(|owner| Some(owner.id.to_string()));
    let owner = owner.as_deref();

    // Check if the user is the owner of the task -> THIS WILL CHANGE to use the JWT token
    // This is a temporary solution until we have JWT authentication
    if owner.is_none() {
        return Response::Unauthorized("Sign in".to_string());
    }

    let is_owner = check_is_owner(task_id, owner.unwrap()).await;

    if is_owner.is_err() {
        let err = is_owner.unwrap_err();
        return match err {
            crate::database::DBReadError::NotFound(_) => Response::NotFound("Task not found".to_string()),
            crate::database::DBReadError::Other(_) => {
                dbg!("Unhandled/Unkown error checking owner: {:?}", err);
                Response::InternalServerError("There was an unkown error".to_string())
            }
        }
    }

    if !is_owner.unwrap() {
        return Response::Forbidden("You do not have permissions".to_string());
    }
    
    // Update the task in the DB
    let updated_task = edit_task_by_id(task_id, title, description, completed_at, owner).await;

    // If there was an error handle it
    if updated_task.is_err() {
        let err = updated_task.unwrap_err();
        return match err {
            crate::database::DBEditError::NotFound(_) => Response::BadRequest("ToDoItem not found".to_string()),
            crate::database::DBEditError::BadData(wrapped_err) => Response::BadRequest(format!("Invalid data: {}", wrapped_err).to_string()),
            crate::database::DBEditError::Other(wrapped_err) => { // If the error is unkown log it and return Status 500
                dbg!("Unkown/Unhandled error when updating a task: {:?}", wrapped_err);
                Response::InternalServerError("There was an unkown error".to_string())
            },
        }
    }
    let task = updated_task.unwrap();

    // Return the edited task
    Response::Ok(Json(task))
    
}

#[delete("/tasks/<task_id>")]
pub async fn delete_task_handler(task_id: &str) -> super::Response<Json<ToDoTask>> {
    // Delete the task
    let deleted_task = delete_task_by_id(task_id).await;

    // If there was an error handle it correctly
    if deleted_task.is_err() {
        let err = deleted_task.unwrap_err();
        return match err {
            crate::database::DBReadError::NotFound(_) => Response::NotFound("Task not found".to_string()),
            crate::database::DBReadError::Other(_) => {
                dbg!("Unhandled/Unkown error deleting task: {:?}", err);
                Response::InternalServerError("There was an unkown error".to_string())
            }
        }
    }
    let task = deleted_task.unwrap();

    // If the task was deleted, return a 200 OK response with the task
    Response::Ok(Json(task))
}