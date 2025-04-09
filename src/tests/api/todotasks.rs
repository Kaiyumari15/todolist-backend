use rocket::local::asynchronous::Client;
use rocket::http::{Status, Header};
use crate::model::users::User;
use crate::model::todotask::ToDoTask;
use crate::database::{connect, clear_all_test};
use super::rocket_test_launch;

#[cfg(test)]
mod creating {
    use super::*;

    #[rocket::async_test]
    /// Test creating a task successfully
    /// This test ensures that a task can be created and the response status is correct.
    async fn test_create_task() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Create a client for sending requests
        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Define a user and sign up to get a token
        let user = User {
            username: Some("test_user".to_string()),
            email: Some("test_user@example.com".to_string()),
            password: Some("password123".to_string()),
            created_at: None,
            id: None,
        };

        // Send a POST request to sign up the user
        let sign_up_response = client
            .post("/users/sign-up")
            .json(&user)
            .dispatch()
            .await;

        // Assert that the response status is Created (201)
        assert_eq!(sign_up_response.status(), Status::Created);

        // Extract the token from the response
        let token = sign_up_response.into_string().await.unwrap(); // Assuming token is returned as plain text

        // Define a task to create
        let task = ToDoTask {
            title: Some("Test Task".to_string()),
            description: Some("A task for testing".to_string()),
            completed_at: None,
            created_at: None,
            id: None,
            owner: None,
        };

        // Send a POST request to create the task
        let response = client
            .post("/tasks")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .json(&task)
            .dispatch()
            .await;

        // Assert that the response status is Created (201)
        assert_eq!(response.status(), Status::Created);
    }

    #[rocket::async_test]
    /// Test creating a task with invalid data
    /// This test ensures that creating a task with missing required fields returns an error.
    async fn test_create_task_invalid_data() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Create a client for sending requests
        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Define a user and sign up to get a token
        let user = User {
            username: Some("test_user".to_string()),
            email: Some("test_user@example.com".to_string()),
            password: Some("password123".to_string()),
            created_at: None,
            id: None,
        };

        // Send a POST request to sign up the user
        let sign_up_response = client
            .post("/users/sign-up")
            .json(&user)
            .dispatch()
            .await;

        // Assert that the response status is Created (201)
        assert_eq!(sign_up_response.status(), Status::Created);

        // Extract the token from the response
        let token = sign_up_response.into_string().await.unwrap(); // Assuming token is returned as plain text

        // Define a task with invalid data (missing title)
        let task = ToDoTask {
            title: None, // Invalid data: missing title
            description: Some("A task for testing".to_string()),
            completed_at: None,
            created_at: None,
            id: None,
            owner: None,
        };

        // Send a POST request to create the task
        let response = client
            .post("/tasks")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .json(&task)
            .dispatch()
            .await;

        // Assert that the response status is BadRequest (400)
        assert_eq!(response.status(), Status::BadRequest);
    }
}

#[cfg(test)]
mod editing {
    use super::*;

    #[rocket::async_test]
    async fn test_edit_task() {
        connect().await;
        clear_all_test().await;

        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Create a user and get the token
        let user = User {
            username: Some("test_user".to_string()),
            email: Some("test_user@example.com".to_string()),
            password: Some("password123".to_string()),
            created_at: None,
            id: None,
        };

        let sign_up_response = client
            .post("/users/sign-up")
            .json(&user)
            .dispatch()
            .await;

        assert_eq!(sign_up_response.status(), Status::Created);

        let token = sign_up_response.into_string().await.unwrap(); // Assuming token is returned as plain text

        let updated_task = ToDoTask {
            title: Some("Updated Task".to_string()),
            description: Some("Updated description".to_string()),
            completed_at: None,
            created_at: None,
            id: None,
            owner: None,
        };

        let response = client
            .patch("/tasks/1")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .json(&updated_task)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }
}

#[cfg(test)]
mod deleting {
    use super::*;

    #[rocket::async_test]
    async fn test_delete_task() {
        connect().await;
        clear_all_test().await;

        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Create a user and get the token
        let user = User {
            username: Some("test_user".to_string()),
            email: Some("test_user@example.com".to_string()),
            password: Some("password123".to_string()),
            created_at: None,
            id: None,
        };

        let sign_up_response = client
            .post("/users/sign-up")
            .json(&user)
            .dispatch()
            .await;

        assert_eq!(sign_up_response.status(), Status::Created);

        let token = sign_up_response.into_string().await.unwrap(); // Assuming token is returned as plain text

        let response = client
            .delete("/tasks/1")
            .header(Header::new("Authorization", format!("Bearer {}", token)))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }
}