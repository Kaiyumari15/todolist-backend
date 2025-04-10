use rocket::{post, patch, delete, serde::json::Json};
use crate::database::todotask::{check_is_owner, create_task, delete_task_by_id, edit_task_by_id};
use crate::model::todotask::ToDoTask;
use super::auth::{verify_token, JWT};
use super::Response;

#[post("/tasks", data = "<input_task>")]
/// Create a new task
/// This function handles the creation of a new task by accepting a JSON payload containing the task's details.
/// 
/// # Arguments
/// * `input_task` - A JSON payload containing the task's details, including title, description, and completed_at.
/// * `jwt` - A JWT token for authentication, which is passed in the request `Authorization` header.
/// 
/// # Returns
/// * `Response<Json<ToDoTask>>` - A response indicating the result of the task creation process. If successful, it returns the created task in JSON format.
pub async fn create_task_handler(
    input_task: Json<ToDoTask>,
    jwt: JWT,
) -> Response<Json<ToDoTask>> {
    let input_task = input_task.into_inner(); // Deserialise the input from JSON

    // Verify the token & extract the user ID from it
    let user_id = verify_token(&jwt.token).await; 
    if user_id.is_err() {
        return Response::Unauthorized("Invalid token".to_string())
    }
    let user_id = user_id.unwrap().sub; // Extract the user ID from the token

    // Option<String> -> Option<&str>
    let title = input_task.title.as_deref();
    let description = input_task.description.as_deref();
    let completed_at = input_task.completed_at.as_deref();
    let created_at = input_task.created_at.as_deref();

    // Check if the title is empty
    if title.is_none() {
        return Response::BadRequest("Title is required".to_string());
    }
    let title = title.unwrap();

    // Create the task 
    let created_task = create_task(&user_id, title, description, completed_at, created_at).await;

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
/// Update an existing task
/// This function handles the update of an existing task by accepting a JSON payload containing the updated task's details.
/// 
/// # Arguments
/// * `task_id` - The ID of the task to be updated.
/// * `update_task` - A JSON payload containing the updated task's details, including title, description, and completed_at.
/// * `jwt` - A JWT token for authentication, which is passed in the request `Authorization` header.
/// 
/// # Returns
/// * `Response<Json<ToDoTask>>` - A response indicating the result of the task update process. If successful, it returns the updated task in JSON format.
pub async fn update_task_handler(task_id: &str, update_task: Json<ToDoTask>, jwt: JWT) -> super::Response<Json<ToDoTask>> {

    // Deserialise the input from JSON
    let update_task = update_task.into_inner();

    // Verify the token and extracrt the user id 
    let user_id = verify_token(&jwt.token).await;
    if user_id.is_err() {
        return Response::Unauthorized("Invalid token".to_string())
    }
    let user_id = user_id.unwrap().sub;

    // Option<String> -> Option<&str>
    let title = update_task.title.as_deref();
    let description = update_task.description.as_deref();
    let completed_at = update_task.completed_at.as_deref();
    let owner = &user_id;

    // Check if the user is the owner of the task -> THIS WILL CHANGE to use the JWT token
    // This is a temporary solution until we have JWT authentication

    let is_owner = check_is_owner(task_id, owner).await;

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
    let updated_task = edit_task_by_id(task_id, title, description, completed_at, Some(owner)).await;

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
/// Delete a task
/// This function handles the deletion of a task by its ID.
/// 
/// # Arguments
/// * `task_id` - The ID of the task to be deleted.
/// * `jwt` - A JWT token for authentication, which is passed in the request `Authorization` header.
/// 
/// # Returns
/// * `Response<Json<ToDoTask>>` - A response indicating the result of the task deletion process. If successful, it returns the deleted task in JSON format.
pub async fn delete_task_handler(task_id: &str, jwt: JWT) -> super::Response<Json<ToDoTask>> {
    // Verify the JWT
    let user_id = verify_token(&jwt.token).await;
    if user_id.is_err() {
        return Response::Unauthorized("Unauthorised".to_string())
    }
    let user_id = user_id.unwrap().sub; // Extract the user ID from the token

    // Check if the user is the owner
    let is_owner = check_is_owner(&user_id, task_id).await;
    if is_owner.is_err() || !is_owner.unwrap() {
        return Response::Forbidden("You are not the owner of the task".to_string())
    }
    
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