use surrealdb::sql::{Value, Thing};

use crate::model::users::User;

use super::{DBCreateError, DBEditError, DBReadError, DB};

pub async fn create_user(username: &str, email: &str, password: &str) -> Result<User, DBCreateError> {
    // Create the query
    let sql = "
    CREATE User SET
    username = $username,
    email = $email,
    password = $password;
    ";

    // Convert the inputs 
    let username = Value::from(username);
    let email = Value::from(email);
    let password = Value::from(password);

    let mut response = DB.query(sql)
        .bind(("username", username))
        .bind(("email", email))
        .bind(("password", password))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    // Take the first result and convert it to a User
    let result: Option<User> = response
        .take(0)
        .map_err(|e| {
            if e.to_string().contains("uniqueUsername") {
                DBCreateError::BadData("Username already exists".to_string())
            } else if e.to_string().contains("uniqueEmail") {
                DBCreateError::BadData("Email already exists".to_string())
            } else {
                DBCreateError::Other(e.to_string())
            }
        })?;

    // Check if the result is None and return an error if it is
    if result.is_none() {
        return Err(DBCreateError::Other("Failed to create user".to_string()));
    }
    let result = result.unwrap();

    // Return the User struct
    Ok(result)
}

pub async fn compare_username_password(username: &str, password: &str) -> Result<User, DBReadError> {
    // Create the query
    let sql = "SELECT * FROM User WHERE username = $username AND password = $password;";

    // Convert the inputs 
    let username = Value::from(username);
    let password = Value::from(password);

    let mut response = DB.query(sql)
        .bind(("username", username))
        .bind(("password", password))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    // Take the first result and convert it to a User
    let result: Option<User> = response
        .take(0)
        .map_err(|e| {
            DBReadError::Other(e.to_string())
        })?;

    // Check if the result is None and return an error if it is
    if result.is_none() {
        return Err(DBReadError::NotFound("Failed to get user".to_string()));
    }
    let result = result.unwrap();

    // Return the User struct
    Ok(result)
}

pub async fn compare_email_password(email: &str, password: &str) -> Result<User, DBReadError> {
    // Create the query
    let sql = "SELECT * FROM User WHERE email = $email AND password = $password;";

    // Convert the inputs 
    let email = Value::from(email);
    let password = Value::from(password);

    let mut response = DB.query(sql)
        .bind(("email", email))
        .bind(("password", password))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    // Take the first result and convert it to a User
    let result: Option<User> = response
        .take(0)
        .map_err(|e| {
            DBReadError::Other(e.to_string())
        })?;

    // Check if the result is None and return an error if it is
    if result.is_none() {
        return Err(DBReadError::NotFound("Failed to get user".to_string()));
    }
    let result = result.unwrap();

    // Return the User struct
    Ok(result)
}

pub async fn delete_user(id: &str) -> Result<User, DBEditError> {
    // Create the query
    let sql = "DELETE ONLY $id RETURN BEFORE;";

    // Convert the id to a surrealdb::sql::value
    // This means I dont have to case anything in the SQL
    // I dont have to explicitly do this but I prefer to
    let id: Value = Thing::from(("User", id)).into();

    let mut response = DB.query(sql)
        .bind(("id", id))
        .await
        .unwrap(); // Its okay if this panics because it will only panic if the database is not connected or the query is malformed

    // Take the first result and convert it to a User
    let result: Option<User> = response
        .take(0)
        .map_err(|e| {
            if e.to_string().contains("ONLY keyword") { // When this is being written there is a bug in surrealdb that causes this error to be thrown when deleting a user which does not exist when using the ONLY keyword
                DBEditError::NotFound("Failed to delete user".to_string());
            }
            DBEditError::Other(e.to_string())
        })?;

    // Check if the result is None and return an error if it is
    if result.is_none() {
        return Err(DBEditError::NotFound("Failed to delete user".to_string()));
    }
    let result = result.unwrap();

    // Return the User struct
    Ok(result)

}