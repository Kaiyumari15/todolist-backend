use chrono::Duration;
use rocket::{post, serde::json::Json};

use crate::{database::users::{compare_email_password, compare_username_password, create_user}, model::users::User};

use super::{auth::generate_token, Response};

#[post("/users/sign-up", data = "<input_task>")]
pub async fn create_user_handler(
    input_task: Json<User>
) -> Response<String> {
    let input_task = input_task.into_inner(); // Deserialise the input from JSON

    // Option<String> -> Option<&str>
    let username = input_task.username.as_deref();
    let password = input_task.password.as_deref();
    let email = input_task.email.as_deref();

    // Check if any of the required fields are empty
    if email.is_none() {
        return Response::BadRequest("Email is required".to_string());
    }
    let email = email.unwrap();
    // Here I should check if the email is valid
    if username.is_none() {
        return Response::BadRequest("Username is required".to_string());
    }
    let username = username.unwrap();
    // I can add username requirements here if I want to
    if password.is_none() {
        return Response::BadRequest("Password is required".to_string());
    }
    let password = password.unwrap();
    // I can add password requirements here if I want to

    // Create the task 
    let created_user = create_user(username, email, password).await;

    // Check if there was an error
    if created_user.is_err() {
        let err = created_user.unwrap_err();
        return match err {
            crate::database::DBCreateError::AlreadyExists(_) => Response::BadRequest("This user already exists".to_string()),
            crate::database::DBCreateError::BadData(_) => Response::BadRequest("The data provided is invalid".to_string()),
            crate::database::DBCreateError::Other(_) => {
                dbg!("Unhandled/Unkown error creating user: {:?}", err);
                Response::InternalServerError("There was an unkown error".to_string())
            }
        }
    }
    let user = created_user.unwrap();
    let id = user.id.unwrap().id.to_string();

    // Generate a JWT for the user
    let duration = Duration::days(7); // The token will be valid for 7 days
    let jwt = generate_token(&id, duration).await;

    // Return the response
    Response::Created(jwt)
}

#[post("/users/log-in", data = "<input_user>")]
pub async fn sign_in_user_handler(
    input_user: Json<User>
) -> Response<String> {
    let user: User;
    let input_user = input_user.into_inner(); // Deserialise the input from JSON

    // Option<String> -> Option<&str>
    let username = input_user.username.as_deref();
    let password = input_user.password.as_deref();
    let email = input_user.email.as_deref();

    // Check there is a password
    if password.is_none() {
        return Response::BadRequest("Password is required".to_string());
    }
    
    // Check there is a username OR password and call the correct function
    if username.is_some() {
        let compare_result = compare_username_password(username.unwrap(), password.unwrap()).await;
        if compare_result.is_err() {
            let err = compare_result.unwrap_err();
            return match err {
                crate::database::DBReadError::NotFound(_) => Response::BadRequest("Incorrect Username/Password".to_string()),
                crate::database::DBReadError::Other(_) => {
                    dbg!("Unhandled/Unkown error logging in user: {:?}", err);
                    Response::InternalServerError("There was an unkown error".to_string())
                }
                _ => Response::InternalServerError("There was an unkown error".to_string())
            }
        }
        user = compare_result.unwrap();
    } else if email.is_some() {
        let compare_result = compare_email_password(email.unwrap(), password.unwrap()).await;
        if compare_result.is_err() {
            let err = compare_result.unwrap_err();
            return match err {
                crate::database::DBReadError::NotFound(_) => Response::BadRequest("Incorrect Email/Password".to_string()),
                crate::database::DBReadError::Other(_) => {
                    dbg!("Unhandled/Unkown error logging in user: {:?}", err);
                    Response::InternalServerError("There was an unkown error".to_string())
                }
                _ => Response::InternalServerError("There was an unkown error".to_string())
            }
        }
        user = compare_result.unwrap();
    } else {
        return Response::BadRequest("Username or email is required".to_string());
    }

    // Generate a JWT for the user
    let id = user.id.unwrap().id.to_string();
    let duration = Duration::days(7); // The token will be valid for 7 days
    let jwt = generate_token(&id, duration).await;

    // Return the response
    Response::Ok(jwt)
}