#[cfg(test)]
mod todotask {
    use chrono::{DurationRound, TimeDelta, Utc};
    
    use crate::database::{connect, todotask::{create_task, delete_task_by_id, edit_task_by_id, get_task_by_id}};
    
    
    #[tokio::test]
    async fn test_create_task_all_fields() {
        let now_str = Utc::now()
            .duration_round(
                TimeDelta::try_milliseconds(10)
                .unwrap())
            .unwrap()
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        
        let _ = connect().await;
        let result = create_task("TESTtitle", Some("TESTdescription"), Some(&now_str), Some(&now_str)).await;
        

        assert!(result.is_ok(), "Failed to create task with all fields: {:?}", result.err());
        let task = result.unwrap();

        // check each of the fields match up
        assert_eq!(task.title, "TESTtitle", "Title mismatch");
        assert_eq!(task.description, Some("TESTdescription".to_string()), "Description mismatch");
        assert_eq!(task.completed_at, Some(now_str.clone()), "Completed_at mismatch");
        assert_eq!(task.created_at, Some(now_str.clone()), "Created_at mismatch");
    }

    #[tokio::test]
    async fn test_create_task_min_fields() {
        let _ = connect().await;
        let result = create_task("TESTtitle", None, None, None).await;

        assert!(result.is_ok(), "Failed to create task with minimum fields: {:?}", result.err());
        let task = result.unwrap();

        // check each of the fields match up
        assert_eq!(task.title, "TESTtitle", "Title mismatch");
        assert!(task.description.is_none(), "Description should be None");
        assert!(task.completed_at.is_none(), "Completed_at should be None");
        assert!(task.created_at.is_some(), "Created_at should not be None");
    }

    #[tokio::test]
    async fn test_create_task_bad_data() {
        let _ = connect().await;
        let result = create_task("Title", None, Some("sOmeBaDdAtA"), None).await;

        assert!(result.is_err(), "Expected error when creating task with bad data: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_get_task_by_id() {
        let _ = connect().await;
        let result = create_task("TESTtitle", Some("TESTdescription"), None, None).await;

        assert!(result.is_ok(), "Failed to create task for get test: {:?}", result.err());
        let task = result.unwrap();

        let result = get_task_by_id(&task.id.id.to_string()).await;
        assert!(result.is_ok(), "Failed to get task by id: {:?}", result.err());
        let task2 = result.unwrap();

        // check each of the fields match up
        assert_eq!(task.title, task2.title, "Title mismatch");
        assert_eq!(task.description, task2.description, "Description mismatch");
        assert_eq!(task.completed_at, task2.completed_at, "Completed_at mismatch");
        assert_eq!(task.created_at, task2.created_at, "Created_at mismatch");
    }

    #[tokio::test]
    async fn test_get_task_by_id_not_found() {
        let _ = connect().await;
        let result = get_task_by_id("nonexistent_id").await;

        assert!(result.is_err(), "Expected error when getting task by nonexistent id: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_delete_task_by_id() {
        let _ = connect().await;

        // Create a task to delete
        let result = create_task("TESTtitle", Some("TESTdescription"), None, None).await;
        assert!(result.is_ok(), "Failed to create task for delete test: {:?}", result.err());
        let task = result.unwrap();

        // Delete the task
        let delete_result = delete_task_by_id(&task.id.id.to_string()).await;
        assert!(delete_result.is_ok(), "Failed to delete task by id: {:?}", delete_result.err());

        // Check the task is deleted
        let get_result = get_task_by_id(&task.id.id.to_string()).await;
        assert!(get_result.is_err(), "Expected error when getting deleted task: {:?}", get_result.err());
    }

    #[tokio::test]
    async fn test_delete_task_by_id_not_found() {
        let _ = connect().await;

        // Attempt to delete a nonexistent task
        let result = delete_task_by_id("nonexistent_id").await;
        assert!(result.is_err(), "Expected error when deleting task by nonexistent id: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_edit_task_by_id_title() {
        let _ = connect().await;

        // Create a task to edit
        let result = create_task("TESTtitle", Some("TESTdescription"), None, None).await;
        assert!(result.is_ok(), "Failed to create task for edit test: {:?}", result.err());
        let task = result.unwrap();

        // Edit the task
        let edit_result = edit_task_by_id(&task.id.id.to_string(), Some("TESTnewtitle"), None, None).await;
        assert!(edit_result.is_ok(), "Failed to edit task by id: {:?}", edit_result.err());

        // Check the task is edited
        let get_result = get_task_by_id(&task.id.id.to_string()).await;
        assert!(get_result.is_ok(), "Failed to get edited task by id: {:?}", get_result.err());
        let edited_task = get_result.unwrap();

        // Check the edited fields match up
        assert_eq!(edited_task.title, "TESTnewtitle", "Title mismatch after edit");
    }

    #[tokio::test]
    async fn test_edit_task_by_id_all() {
        let _ = connect().await;

        // Create a task to edit
        let result = create_task("TESTtitle", Some("TESTdescription"), None, None).await;
        assert!(result.is_ok(), "Failed to create task for edit test: {:?}", result.err());
        let task = result.unwrap();

        // Edit all fields of the task
        let new_completed_at = Utc::now()
            .duration_round(
                TimeDelta::try_milliseconds(10)
                .unwrap())
            .unwrap()
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let edit_result = edit_task_by_id(&task.id.id.to_string(), Some("TESTnewtitle"), Some("TESTnewdescription"), Some(&new_completed_at)).await;
        assert!(edit_result.is_ok(), "Failed to edit task by id: {:?}", edit_result.err());

        // Check the task is edited
        let get_result = get_task_by_id(&task.id.id.to_string()).await;
        assert!(get_result.is_ok(), "Failed to get edited task by id: {:?}", get_result.err());
        let edited_task = get_result.unwrap();

        // Check the edited fields match up
        assert_eq!(edited_task.title, "TESTnewtitle".to_string(), "Title mismatch after edit");
        assert_eq!(edited_task.description, Some("TESTnewdescription".to_string()), "Description mismatch after edit");
        assert_eq!(edited_task.completed_at, Some(new_completed_at), "Completed_at mismatch after edit");
    }

}