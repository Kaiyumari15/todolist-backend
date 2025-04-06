#[cfg(test)]
mod generating {
    use chrono::Duration;

    use crate::api::auth::generate_token;
    
    #[tokio::test]
    /// Test the token generation function does not panic
    /// We do not test any data relating to the token, this is done in verifying
    async fn generate_token_successful() {
        let user_id = "some_fake_id";
        let duration = Duration::hours(1); // 1 hour in seconds
        
        let _ = generate_token(user_id, duration).await;
    }
}

#[cfg(test)]
mod verifying {
    use std::any::Any;

    use crate::api::auth::{generate_token, verify_token, VerifyJWTError};

    #[tokio::test]
    /// Test the token verification function does not return an error
    /// Also test the token is valid and not expired
    async fn verify_token_successful() {
        // Generate a token for a user with a duration of 1 hour
        let user_id = "some_fake_id";
        let duration = chrono::Duration::hours(1); // 1 hour in seconds
        let token = generate_token(user_id, duration).await;

        // Verify the token
        let result = verify_token(&token).await;
        assert!(result.is_ok(), "Token verification failed: {:?}", result.err());

        // Verify the user id in the token matches the expected user id
        let claims = result.unwrap();
        assert_eq!(claims.sub, user_id, "Token user id does not match expected user id");

    }

    #[tokio::test]
    /// Test the token verification function returns an error when the token is expired
    async fn verify_token_expired() {
        // Generate a token for a user with a duration of 1 second
        let user_id = "some_fake_id";
        let duration = chrono::Duration::seconds(1); // 1 second in seconds
        let token = generate_token(user_id, duration).await;

        // Wait for the token to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Verify the token
        let result = verify_token(&token).await;
        assert!(result.is_err(), "Token verification should have failed but succeeded: {:?}", result.err());
        
        // Check the error message
        let error = result.err().unwrap();
        assert_eq!(error, VerifyJWTError::Expired, "Token verification error type does not match expected type");
    }
}