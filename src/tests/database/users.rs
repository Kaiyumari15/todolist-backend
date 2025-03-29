mod create {
    use crate::database::{connect, users::create_user};


    #[tokio::test]
    async fn create_user_successfull() {
        // Connect to the DB
        let _ = connect().await;

        // Create the user
        let user = create_user("TESTuser", "callum@example.com", "Password123").await.expect("Failed to create user");

        // Check the fields are correct
        assert_eq!(user.username, Some("TESTuser".to_string()), "Field mismatch for username");
        assert_eq!(user.email, Some("callum@example.com".to_string()), "Field mismatch for email");
        assert_eq!(user.password, Some("Password123".to_string()), "Field mismatch for password");
    }
}

mod delete {
    use crate::database::{connect, users::{create_user, delete_user}};

    #[tokio::test]
    async fn delete_user_successfull() {

        // Connect to the DB
        let _ = connect().await;   
        
        // Create a user to delete
        let created = create_user("TESTuser", "test@example.com", "Password123").await.expect("Failed to create user");
        
        // Delete the user
        let deleted = delete_user(&created.id.unwrap().id.to_string()).await.expect("Failed to delete user");
        dbg!(&deleted);

        assert_eq!(deleted.username, Some("TESTuser".to_string()), "Field mismatch for username");
        assert_eq!(deleted.email, Some("test@example.com".to_string()), "Field mismatch for email");
        assert_eq!(deleted.password, Some("Password123".to_string()), "Field mismatch for password");
    }
}