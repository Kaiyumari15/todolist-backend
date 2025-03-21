use crate::model::todotask::ToDoTask;

use super::{DBCreateError, DB};

pub async fn create_task(
    title: String,
    description: Option<String>,
    completed_at: Option<String>,
    created_at: Option<String>,
) -> Result<ToDoTask, DBCreateError> {

    let sql = "
    CREATE ToDoTask
    SET title = $title,
    description = $description,
    completed_at = $completed_at,
    created_at = $created_at;
    ";

    let mut response = DB.query(sql)
        .bind(("title", title))
        .bind(("description", description))
        .bind(("completed_at", completed_at))
        .bind(("created_at", created_at))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    let result: Option<ToDoTask> = response
        .take(0)
        .map_err(|e| {
            DBCreateError::Other(e.to_string())
        })?;
        
    let result = result.ok_or_else(|| {
        DBCreateError::Other("Failed to create task".to_string())
    })?;
    Ok(result)
}