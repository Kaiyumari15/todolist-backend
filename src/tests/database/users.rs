
#[cfg(test)]
mod creating {
    use crate::database::users::create_user;
    use crate::database::{DB, connect, clear_all_test};

    #[tokio::test]
    /// Test creating a user
    pub async fn create_user_successfull() {
        // Connect to the database and clear test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Check there were no errors
        assert!(user.is_ok(), "Failed to create user: {:?}", user.err());
        let user = user.unwrap();
    }

    #[tokio::test]
    /// Test creating a user with an existing username
    pub async fn create_user_duplicate_username() {
        // Connect to the DB and clear existing test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user
        let user1 = create_user("TESTuser", "TEST1@example.com", "TESTpassword").await;

        // Check there were no errors
        assert!(user1.is_ok(), "Failed to create user1: {:?}", user1.err());

        // Create another user with the same username
        let user2 = create_user("TESTuser", "TEST2@example.com", "TESTpassword").await;
        
        // Check there was an error
        assert!(user2.is_err(), "Expected error when creating user with duplicate username")
    }

    #[tokio::test]
    /// Test creating a user with an existing username
    pub async fn create_user_duplicate_email(){
        // Connect to the DB and clear existing test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user
        let user1 = create_user("TESTuser1", "TEST@example.com", "TESTpassword").await;

        // Check there were no errors
        assert!(user1.is_ok(), "Failed to create user1: {:?}", user1.err());

        // Create another user with the same email
        let user2 = create_user("TESTuser2", "TEST@example.com", "TESTpassword").await;
        
        // Check there was an error
        assert!(user2.is_err(), "Expected error when creating user with duplicate email")
    }
}

#[cfg(test)]
mod editing {

}

#[cfg(test)]
mod signing_in {

}

#[cfg(test)]
mod deleting {

}