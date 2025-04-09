#[cfg(test)]
mod creating {
    use crate::database::{connect, clear_all_test, DB, todotask::create_task, users::create_user};

    #[tokio::test]
    /// Test creating a todo task
    /// This test will create a user and then create a todo task for that user.
    /// It will then check if the task was created successfully and if the fields are correct.
    async fn create_todotask_successfull() {
        // Connect to the database and clear everything related to previous tests
        let _ = connect().await;
        let _ = clear_all_test().await;

        // Create a user
        let user = create_user("TESTuser", "TESTemail@example.com", "TESTpassword").await.expect("Failed to create user: ");
        let user_id = user.id.unwrap().id.to_string();

        // Create a todo task
        let task = create_task(&user_id, "TESTtask", Some("TESTdescription"), None, None).await;

        // Assert that the todo task was created successfully
        assert!(task.is_ok(), "Failed to create todo task: {:?}", task.err());
        let task = task.unwrap();

        // Check the fields
        assert_eq!(task.title, Some("TESTtask".to_string()), "Title does not match");
        assert_eq!(task.description, Some("TESTdescription".to_string()), "Description does not match");
        assert_eq!(task.owner.unwrap().id.to_string(), user_id.to_string(), "Owner ID does not match");
    }
}

mod reading {
    
}

mod editing {

}

mod deleting {

}