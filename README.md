# todolist backend

 Rust backend using SurrealDB and Rocket.rs for my todolist app, 21/03/2025 ->

## Documentation

### Data Structures (src/model)

#### ToDoTask

Represents each ToDo Task

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Represents a ToDo task
/// with an ID, title, description, completion status, and timestamps.
/// id: Unique identifier for the task given by SurrealDB
/// title: Title of the task
/// description: Optional description of the task
/// completed_at: Optional timestamp indicating when the task was completed, if is None then the task is assumed to be uncompleted
/// created_at: Timestamp indicating when the task was created
pub struct ToDoTask {
    pub id: Thing,
    pub title: String,
    pub description: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
}
```

### Database (src/database)

#### Connecting to the database

The function `connect()` connects the static variable `DB` to the database.

```rust
pub async fn connect() -> () { /* clipped */ }
```

#### Setting up the database

The function `create_all()` creates all necessary tables in the database.

```rust
pub async fn create_all() -> () { /* clipped */ }
```

#### Creating ToDoTasks

To create ToDoTasks this function should be used.

```rust
pub async fn create_task(
    title: &str,
    description: Option<&str>,
    completed_at: Option<&str>,
    created_at: Option<&str>,
) -> Result<ToDoTask, DBCreateError> { /* clipped */ }
```

#### Getting ToDoTasks

To read ToDoTasks from the database this function should be used

```rust
pub async fn get_task_by_id(
    id: &str,
) -> Result<ToDoTask, DBReadError> { /* clipped */ } 
```

#### Editing ToDoTasks

To edit ToDoTasks in the database this function should be used

```rust
pub async fn edit_task_by_id(
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    completed_at: Option<&str>,
) -> Result<ToDoTask, DBEditError> { /* clipped */ }
```

#### Deleting ToDoTasks

To delete ToDoTasks from the database by id this function should be used

```rust
pub async fn delete_task_by_id(
    id: &str,
) -> Result<ToDoTask, DBReadError> { /* clipped */ }
```

#### Error Types

##### DBCreateError

The error type which will be returned when creating a task in the database.
The String field should contain more information about the error

```rust
#[derive(Debug, Clone)]
pub enum DBCreateError {
    Permissions(String),
    AlreadyExists(String),
    BadData(String),
    Other(String)
}

```

##### DBEditError

The error type which will be returned when editing a task in the database.
The String field should contain more information about the error

```rust
#[derive(Debug, Clone)]
pub enum DBEditError {
    Permissions(String),
    NotFound(String),
    BadData(String),
    Other(String)
}
```

##### DBReadError

The error type which will be returned when reading a task in the database.
The String field should contain more information about the error

```rust
#[derive(Debug, Clone)]
pub enum DBReadError {
    Permissions(String),
    NotFound(String),
    Other(String),
}

```

### API routes

### Unit Tests

Documentation / Explanations for each of the unit tests in the project.

#### database\connect

These unit tests are for testing database functions relevant to connecting to the database and setting it up for use.

The first test tests that the database connects successfully.

```rust
    #[tokio::test]
    async fn test_connect_success() {
        // Assuming the database is running
        connect().await;
        // If no panic occurred, the connection was successful
    }
```

There is no need for assertions here, as any fail in the function will result in a panic. This is okay in this case because this function will only be called during development and when deploying for production, at all of these times it is a critical error.

The next test that the database creates tables successfully.

```rust
    #[tokio::test]
    async fn test_create_all_success() {
        // Assuming the database is running
        connect().await;
        create_all().await;
        // If no panic occurred, the table and fields were created successfully
    }
```

There is no need for assertions here, as any fail in the function will result in a panic. This is okay in this case because this function will only be called during development and when deploying for production, at all of these times it is a critical error.

#### database\todotask

The first test checks that a `ToDoTask` can be created in the database when all fields are given to the function.

```rust
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
```

This test checks that a `ToDoTask` can be created using the minimum number of required fields

```rust
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
```

This test checks that a `ToDoTask` cannot be created with bad data

```rust
    #[tokio::test]
    async fn test_create_task_bad_data() {
        let _ = connect().await;
        let result = create_task("Title", None, Some("sOmeBaDdAtA"), None).await;

        assert!(result.is_err(), "Expected error when creating task with bad data: {:?}", result.err());
    }
```

This test checks that a `ToDoTask` can be fetched using the ID

```rust
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
```

A test for `get_task_by_id` ensuring the correct error is given if the id does not exist

```rust
    #[tokio::test]
    async fn test_get_task_by_id_not_found() {
        let _ = connect().await;
        let result = get_task_by_id("nonexistent_id").await;

        assert!(result.is_err(), "Expected error when getting task by nonexistent id: {:?}", result.err());
    }
```

A test for `delete_task_by_id` ensuring the function correctly deletes ToDoTasks from the database

```rust
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
```

A test for `delete_task_by_id` ensuring the function returns an error when editing a non-existent record

```rust
    #[tokio::test]
    async fn test_delete_task_by_id_not_found() {
        let _ = connect().await;

        // Attempt to delete a nonexistent task
        let result = delete_task_by_id("nonexistent_id").await;
        assert!(result.is_err(), "Expected error when deleting task by nonexistent id: {:?}", result.err());
    }
```

A test for `edit_task_by_id` ensuring the function correctly edits the title

```rust
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
```

A test for `edit_task_by_id` ensuring the function correctly edits all fields

```rust
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
```
