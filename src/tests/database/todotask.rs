#[cfg(test)]
mod todotask {
    use chrono::{DurationRound, TimeDelta, Utc};
    
    use crate::database::{todotask::create_task, connect};
    
    
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
}