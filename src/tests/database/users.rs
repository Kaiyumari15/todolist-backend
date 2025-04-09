#[cfg(test)]
mod creating {
    use crate::database::{clear_all_test, connect, users::create_user};

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
    use crate::database::users::{create_user, edit_existing_user};
    use crate::database::{connect, clear_all_test};

    #[tokio::test]
    /// Test correctly updating user information
    async fn update_user_successfully() {
        // Connect to the database and clear existing test data
        let _ = connect().await;
        let _ = clear_all_test().await;
        
        // Create a user to edit
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Ensure there are no errors
        assert!(user.is_ok(), "Couldn't create user: {:?}", user.err());
        let user = user.unwrap();

        // Edit the user
        let id = user.id.unwrap().id.to_string();
        let edited = edit_existing_user(&id, Some("TESTuserNEW"), Some("TESTnew@example.com"), Some("TESTnewpassword")).await;

        // Ensure there are no errors
        assert!(edited.is_ok(), "Couldn't edit user: {:?}", edited.err());
    }

    #[tokio::test]
    /// Test calling the function with nothing to change
    async fn update_user_none() {
        // Connect to the database and clear all test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user to edit
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Check there are no errors
        assert!(user.is_ok(), "Couldn't create user: {:?}", user.err());
        let user = user.unwrap();

        // Edit the user with nothing in the function
        let id = user.id.unwrap().id.to_string();
        let edited = edit_existing_user(&id, None, None, None).await;

        // Check there is an error
        assert!(edited.is_err(), "Expected error when updating user with nothing")
    }
}

#[cfg(test)]
mod signing_in {
    use crate::database::users::{create_user, compare_email_password, compare_username_password};
    use crate::database::{connect, clear_all_test};
    #[allow(dead_code)]
    async fn sign_in_username_password_correct() {
        // Connect to the database and clear test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user to sign in
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Check there are no errors
        assert!(user.is_ok(), "Failed to create user: {:?}", user.err());

        // Sign in the user
        let compare = compare_username_password("TESTuser", "TESTpassword").await;

        // Check there are no errors
        assert!(compare.is_ok(), "Failed to sign in user: {:?}", compare.err());
    }

    #[allow(dead_code)]
    async fn sign_in_email_password_correct() {
        // Connect to the database and clear test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user to sign in
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Check there are no errors
        assert!(user.is_ok(), "Failed to create user: {:?}", user.err());

        // Sign in the user
        let compare = compare_email_password("TEST@example.com", "TESTpassword").await;

        // Check there are no errors
        assert!(compare.is_ok(), "Failed to sign in user: {:?}", compare.err());
    }

    #[allow(dead_code)]
    async fn sign_in_username_password_incorrect() {
        // Connect to the database and clear test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user to sign in
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Check there are no errors
        assert!(user.is_ok(), "Failed to create user: {:?}", user.err());

        // Sign in the user with incorrect password
        let compare = compare_username_password("TESTuser", "WRONGpassword").await;

        // Check there is an error
        assert!(compare.is_err(), "Expected error when signing in with incorrect password");
    }

    #[allow(dead_code)]
    async fn sign_in_email_password_incorrect() {
        // Connect to the database and clear test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user to sign in
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Check there are no errors
        assert!(user.is_ok(), "Failed to create user: {:?}", user.err());

        // Sign in the user with incorrect password
        let compare = compare_email_password("TEST@example.com", "WRONGpassword").await;

        // Check there is an error
        assert!(compare.is_err(), "Expected error when signing in with incorrect password");
    }
}

#[cfg(test)]
mod deleting {
    use crate::database::users::{create_user, delete_user};
    use crate::database::{connect, clear_all_test};

    #[tokio::test]
    /// Test deleting a user successfully
    async fn delete_user_successfully() {
        // Connect to the database and clear test data
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user to delete
        let user = create_user("TESTuser", "TEST@example.com", "TESTpassword").await;

        // Ensure there are no errors
        assert!(user.is_ok(), "Couldn't create user: {:?}", user.err());
        let user = user.unwrap();

        // Delete the user
        let id = user.id.unwrap().id.to_string();
        let deleted = delete_user(&id).await;

        // Ensure there are no errors
        assert!(deleted.is_ok(), "Couldn't delete user: {:?}", deleted.err());
    }
}