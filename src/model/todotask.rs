use surrealdb::sql::Thing;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// Represents a ToDo task in the database
/// 
/// # Fields
/// * `id` - The ID of the task
/// * `title` - The title of the task
/// * `description` - The description of the task
/// * `owner` - The owner of the task
/// * `completed_at` - The date and time when the task was completed
/// * `created_at` - The date and time when the task was created
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