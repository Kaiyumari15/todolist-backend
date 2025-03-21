#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Represents a ToDo task
/// with an ID, title, description, completion status, and timestamps.
/// id: Unique identifier for the task given by SurrealDB
/// title: Title of the task
/// description: Optional description of the task
/// completed_at: Optional timestamp indicating when the task was completed, if is None then the task is assumed to be uncompleted
/// created_at: Timestamp indicating when the task was created
pub struct ToDoTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
}