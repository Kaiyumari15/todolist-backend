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
    title: String,
    description: Option<String>,
    completed_at: Option<String>,
    created_at: Option<String>,
) -> Result<ToDoTask, DBCreateError> { /* clipped */ }
```

#### Getting ToDoTasks

To read ToDoTasks from the database this function should be used

```rust
pub async fn get_task_by_id(
    id: String,
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
