use surrealdb::sql::Thing;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Represents a ToDo task
/// with an ID, title, description, completion status, and timestamps.
/// id: Unique identifier for the task given by SurrealDB
/// title: Title of the task
/// description: Optional description of the task
/// completed_at: Optional timestamp indicating when the task was completed, if is None then the task is assumed to be uncompleted
/// created_at: Timestamp indicating when the task was created
pub struct ToDoTask {
    pub id: Option<Thing>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub owner: Option<Thing>,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
}

impl std::fmt::Display for ToDoTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ToDoTask {{ id: {:?}, title: {:?}, description: {:?}, completed_at: {:?}, created_at: {:?} }}",
            self.id, self.title, self.description, self.completed_at, self.created_at
        )
    }
}