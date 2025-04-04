use chrono::{DateTime, Utc};
use surrealdb::sql::{Value, Datetime as sdbDateTime, Thing};

use crate::model::todotask::ToDoTask;
use super::{DBCreateError, DBEditError, DBReadError, DB};

pub async fn create_task(
    owner: &str,
    title: &str,
    description: Option<&str>,
    completed_at: Option<&str>,
    created_at: Option<&str>,
) -> Result<ToDoTask, DBCreateError> {

    let owner: Value = Thing::from(("User", owner)).into();
    
    let sql = String::from("
    CREATE ToDoTask
    SET title = $title,
    description = $description,
    completed_at = $completed_at,
    created_at = $created_at,
    owner = $owner;
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
        .bind(("description", description)) 
        .bind(("completed_at", completed_at))
        .bind(("created_at", created_at))
        .bind(("owner", owner))
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

pub async fn get_all_tasks_by_user(
    user_id: &str,
) -> Result<Vec<ToDoTask>, DBReadError> {

    // Make the SQL statement
    let sql = "SELECT * FROM ToDoTask WHERE owner = $owner;";

    // Convert the id to a surrealdb::sql::value
    let owner: Value = Thing::from(("User", user_id)).into();

    // Make the query and bind the id to the SQL statement
    let mut response = DB.query(sql)
        .bind(("owner", owner))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    // Take the response and convert it to a Vec<ToDoTask>
    let result: Vec<ToDoTask> = response
        .take(0)
        .map_err(|e| {
            DBReadError::Other(e.to_string())
        })?;

    // Return the vec of tasks
    Ok(result)

}

pub async fn edit_task_by_id(
    id: &str,
    title: Option<&str>,
    description: Option<&str>,
    completed_at: Option<&str>,
    owner: Option<&str>,
) -> Result<ToDoTask, DBEditError> {

    // Check if the user is the owner of the task
    // This is done in the SQL statement so that we dont have to do a second query to check if the user is the owner
    let check_perms_sql = String::from("IF $id.owner != $owner THEN throw \"Permissions('You do not have permission to edit this task') END;");

    let mut sql = String::from("UPDATE $id SET ");

    // Convert the times to a chrono::DateTime<Utc>, leaving None untouched
    let completed_at: Option<DateTime<Utc>> = match completed_at {
        Some(c) => {
            Some(DateTime::parse_from_rfc3339(c)
                .map_err(|e| {
                    DBEditError::BadData(format!("Couldn't format completed_at: {}", e.to_string()))
                })?
                .with_timezone(&Utc))
        },
        None => None,
    };

    // Take each value and make it a surrealdb::sql::value, if optional values are None then we set them to Value::Null
    // I do this so i dont have to cast the type in the SQL statement, because that causes problems if the value is None
    // This lets me keep the actual SQL as simple as possible
    // This is the same as in create_task but we dont need created_at here
    // Also create the sql string here depending on what parameters are passed in 
    let title = match title {
        Some(t) => {
            sql.push_str("title = $title, ");
            Value::from(t)
        },
        None => Value::None,
    };
    let description = match description {
        Some(d) => {
            sql.push_str("description = $description, ");
            Value::from(d)
        },
        None => Value::None,
    };
    let completed_at = match completed_at {
        Some(c) => {
            sql.push_str("completed_at = $completed_at, ");
            Value::Datetime(sdbDateTime::from(c))
        },
        None => Value::None,
    };
    let owner = match owner {
        Some(o) => {
            sql.push_str("owner = $owner, ");
            Value::Thing(Thing::from(("User", o)))
        },
        None => Value::None,
    };

    // Remove the last comma and space from the SQL string
    sql.pop();
    sql.pop();
    // Add a semicolon to the end of the SQL string 
    sql.push_str(";");

    let final_sql = format!("BEGIN TRANSACTION;
    {}
    {}
    COMMIT TRANSACTION;", check_perms_sql, sql);

    // Convert the id to a surrealdb::sql::value
    // This means I dont have to case anything in the SQL
    // I dont have to explicitly do this but I prefer to
    let id: Value = Thing::from(("ToDoTask", id)).into();

    let mut response = DB.query(final_sql)
        .bind(("id", id))
        .bind(("title", title))
        .bind(("description", description)) 
        .bind(("completed_at", completed_at))
        .bind(("owner", owner))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    let result: Option<ToDoTask> = response
        .take(0)
        .map_err(|e| {
            DBEditError::Other(e.to_string())
        })?;

    let result = result.ok_or_else(|| {
        DBEditError::NotFound("Failed to get task".to_string())
    })?;
    
    Ok(result)
}

pub async fn delete_task_by_id(
    id: &str,
) -> Result<ToDoTask, DBReadError> {

    let sql = "DELETE ONLY $id RETURN BEFORE;";

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
        DBReadError::NotFound("Failed to delete task".to_string())
    })?;

    Ok(result)
}

pub async fn check_is_owner(requester_id: &str, task_id: &str) -> Result<bool, DBReadError> {
    let sql = "SELECT owner FROM $id;";

    // Convert the id to a surrealdb::sql::value
    // This means I dont have to case anything in the SQL
    // I dont have to explicitly do this but I prefer to
    let id: Value = Thing::from(("ToDoTask", task_id)).into();

    let mut response = DB.query(sql)
        .bind(("id", id))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

        // get the owner of the task
    let result: Option<ToDoTask> = response
        .take(0)
        .map_err(|e| {
            DBReadError::Other(e.to_string())
        })?;
        
        // check if the result is None and return an error if it is
    let result = result.ok_or_else(|| {
        DBReadError::NotFound("Failed to get task".to_string())
    })?;

        // check if the owner of the task is the same as the requester
    Ok(result.owner.unwrap().id.to_string() == requester_id.to_string())
}