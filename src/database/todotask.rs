use crate::model::todotask::ToDoTask;

use super::{DBCreateError, DBReadError, DB};

pub async fn create_task(
    title: &str,
    description: Option<&str>,
    completed_at: Option<&str>,
    created_at: Option<&str>,
) -> Result<ToDoTask, DBCreateError> {

    let sql = "
    CREATE ToDoTask
    SET title = $title,
    description = $description,
    completed_at = time::round(<datetime>$completed_at, 10ms),
    created_at = time::round(<datetime>$created_at, 10ms);
    ";

    let mut response = DB.query(sql)
        .bind(("title", title.to_owned()))
        .bind(("description", description.and_then(|d| Some(d.to_owned())))) // Here I just change the &str to a String, leaving a None value untouched
        .bind(("completed_at", completed_at.and_then(|c| Some(c.to_owned()))))
        .bind(("created_at", created_at.and_then(|c| Some(c.to_owned()))))
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

pub async fn get_task_by_id(
    id: String,
) -> Result<ToDoTask, DBReadError> {
    let id = format!("ToDoTask:{}", id);

    let sql = "SELECT * FROM $id;";
    let mut response = DB.query(sql)
        .bind(("id", id))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    let result: Option<ToDoTask> = response
        .take(0)
        .map_err(|e| {
            DBReadError::Other(e.to_string())
        })?;
        
    let result = result.ok_or_else(|| {
        DBReadError::Other("Failed to get task".to_string())
    })?;

    Ok(result)
}