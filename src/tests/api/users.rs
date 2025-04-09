use rocket::local::asynchronous::Client;
use crate::model::users::User;
use crate::database::{clear_all_test, connect};
use super::rocket_test_launch;

#[cfg(test)]
mod user_tests {
    use rocket::http::Status;

    use super::*;

    #[rocket::async_test]
    /// Test creating a user successfully
    /// This test ensures that a user can be created and the response status is correct.
    async fn test_create_user() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Create a client for sending requests
        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Define a user to create
        let user = User {
            username: Some("test_user".to_string()),
            email: Some("test_user@example.com".to_string()),
            password: Some("password123".to_string()),
            created_at: None,
            id: None,
        };

        // Send a POST request to create the user
        let response = client
            .post("/users/sign-up")
            .json(&user)
            .dispatch()
            .await;

        // Assert that the response status is Created (201)
        assert_eq!(response.status(), Status::Created);
    }

    #[rocket::async_test]
    /// Test signing in a user successfully
    /// This test ensures that a user can sign in with valid credentials.
    async fn test_sign_in_user() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Create a client for sending requests
        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Define a user to sign in
        let user = User {
            username: Some("test_user".to_string()),
            email: None,
            password: Some("password123".to_string()),
            created_at: None,
            id: None,
        };

        // Send a POST request to log in the user
        let response = client
            .post("/users/log-in")
            .json(&user)
            .dispatch()
            .await;

        // Assert that the response status is OK (200)
        assert_eq!(response.status(), Status::Ok);
    }

    #[rocket::async_test]
    /// Test signing in a user with invalid credentials
    /// This test ensures that signing in with incorrect credentials returns the correct error.
    async fn test_sign_in_user_invalid_credentials() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Create a client for sending requests
        let client = Client::tracked(rocket_test_launch()).await.expect("valid rocket instance");

        // Define a user with invalid credentials
        let user = User {
            username: Some("nonexistent_user".to_string()), // Invalid username
            email: None,
            password: Some("wrongpassword".to_string()), // Invalid password
            created_at: None,
            id: None,
        };

        // Send a POST request to log in the user
        let response = client
            .post("/users/log-in")
            .json(&user)
            .dispatch()
            .await;

        // Assert that the response status is BadRequest (400)
        assert_eq!(response.status(), Status::BadRequest);
    }
}