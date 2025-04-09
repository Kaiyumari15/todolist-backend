use rocket::local::asynchronous::Client;
use crate::api::auth::{generate_token, verify_token};
use crate::database::{connect, clear_all_test, clear_test_data};

#[cfg(test)]
mod token_tests {
    use super::*;

    #[rocket::async_test]
    /// Test generating and verifying a valid token
    /// This test ensures that a token can be generated and verified successfully.
    async fn test_generate_and_verify_token() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Generate a token for a test user
        let user_id = "test_user";
        let token = generate_token(user_id, chrono::Duration::days(1)).await;

        // Verify the generated token
        let claims = verify_token(&token).await;
        assert!(claims.is_ok());

        // Assert that the token's subject matches the user ID
        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id);
    }

    #[rocket::async_test]
    /// Test verifying an expired token
    /// This test ensures that an expired token cannot be verified successfully.
    async fn test_verify_expired_token() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Generate an expired token for a test user
        let user_id = "test_user";
        let token = generate_token(user_id, chrono::Duration::seconds(-1)).await;

        // Verify the expired token
        let claims = verify_token(&token).await;
        assert!(claims.is_err());
    }

    #[rocket::async_test]
    /// Test verifying a malformed token
    /// This test ensures that a malformed token cannot be verified successfully.
    async fn test_verify_malformed_token() {
        // Connect to the database
        connect().await;
        // Clear all test data
        clear_all_test().await;

        // Define a malformed token
        let malformed_token = "this.is.not.a.valid.token";

        // Verify the malformed token
        let claims = verify_token(malformed_token).await;
        assert!(claims.is_err());
    }
}