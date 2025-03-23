use chrono::{DateTime, Utc};
use surrealdb::sql::{Value, Datetime as sdbDateTime, Thing};

use crate::model::todotask::ToDoTask;
use super::{DBCreateError, DBReadError, DB};

pub async fn create_task(
    title: &str,
    description: Option<&str>,
    completed_at: Option<&str>,
    created_at: Option<&str>,
) -> Result<ToDoTask, DBCreateError> {

    let mut sql = String::from("
    CREATE ToDoTask
    SET title = $title,
    description = $description,
    completed_at = $completed_at,
    created_at = $created_at;
    ");

    // Convert the times to a chrono::DateTime<Utc>, leaving None untouched
    let completed_at: Option<DateTime<Utc>> = match completed_at {
        Some(c) => {
            Some(DateTime::parse_from_rfc3339(c)
                .map_err(|e| {
                    DBCreateError::BadData(format!("Couldn't format completed_at: {}", e.to_string()))
                })?
                .with_timezone(&Utc))
        },
        None => None,
    };
    let created_at: Option<DateTime<Utc>> = match created_at {
        Some(c) => {
            Some(DateTime::parse_from_rfc3339(c)
                .map_err(|e| {
                    DBCreateError::BadData(format!("Couldn't format created_at: {}", e.to_string()))
                })?
                .with_timezone(&Utc))
        },
        None => None,
    };

    // Take each value and make it a surrealdb::sql::value, if optional values are None then we set them to Value::Null
    // I do this so i dont have to cast the type in the SQL statement, because that causes problems if the value is None
    // This lets me keep the actual SQL as simple as possible
    let title: Value = Value::from(title);
    let description = match description {
        Some(d) => Value::from(d),
        None => Value::None,
    };
    let completed_at = match completed_at {
        Some(c) => Value::Datetime(sdbDateTime::from(c)),
        None => Value::None,
    };
    let created_at = match created_at {
        Some(c) => Value::Datetime(sdbDateTime::from(c)),
        None => Value::None,
    };

    let mut response = DB.query(sql)
        .bind(("title", title))
        .bind(("description", description)) // Here I just change the &str to a String, leaving a None value untouched
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

pub async fn get_task_by_id(
    id: &str,
) -> Result<ToDoTask, DBReadError> {

    let sql = "SELECT * FROM $id;";

    // Convert the id to a surrealdb::sql::value
    // This means I dont have to case anything in the SQL
    // I dont have to explicitly do this but I prefer to
    let id: Value = Thing::from(("ToDoTask", id)).into();

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
        DBReadError::NotFound("Failed to get task".to_string())
    })?;

    Ok(result)
}