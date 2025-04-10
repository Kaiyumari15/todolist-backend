use surrealdb::sql::{Value, Thing};

use crate::model::users::User;

use super::{DBCreateError, DBEditError, DBReadError, DB};


/// Create a new user in the database
/// 
/// # Arguments
/// * `username` - The username of the user
/// * `email` - The email of the user
/// * `password` - The password of the user
/// 
/// # Returns
/// `Result<User, DBCreateError>` - The created user or an error
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

/// Test a username/password combination 
/// 
/// # Arguments
/// * `username` - The username of the user
/// * `password` - The password of the user
/// 
/// # Returns
/// `Result<User, DBReadError>` - The user if the username and password are correct, or an error
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

/// Test a email/password combination returning the User if correct
/// 
/// # Arguments
/// * `email` - The email of the user
/// * `password` - The password of the user
/// 
/// # Returns
/// `Result<User, DBReadError>` - The user if the email and password are correct, or an error
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

#[allow(dead_code)]
/// Edit a user from the database by id
/// 
/// # Arguments
/// * `id` - The id of the user to edit
/// * `username` - The new username of the user
/// * `email` - The new email of the user
/// * `password` - The new password of the user
/// 
/// # Returns 
/// `Result<User, DBEditError>` - The edited user or an error
pub async fn edit_existing_user(id: &str, username: Option<&str>, email: Option<&str>, password: Option<&str>) -> Result<User, DBEditError> {

    // Check not all inputs are NONE as this will create an invalid SQL statement
    if email.is_none() && username.is_none() && password.is_none() {
        return Err(DBEditError::BadData("Nothing to change".to_string()))
    }

    // Start the sql query
    let mut sql = "UPDATE $id SET ".to_string();

    // Take each value and make it a surrealdb::sql::value, if optional values are None then we set them to Value::Null
    // I do this so i dont have to cast the type in the SQL statement, because that causes problems if the value is None
    // This lets me keep the actual SQL as simple as possible
    // This is the same as in create_task but we dont need created_at here
    // Also create the sql string here depending on what parameters are passed in 
    let email = match email {
        Some(e) => {
            sql.push_str("email = $email, ");
            Value::from(e)
        },
        None => Value::None,
    };
    let username = match username {
        Some(u) => {
            sql.push_str("username = $username, ");
            Value::from(u)
        },
        None => Value::None,
    };
    let password = match password {
        Some(p) => {
            sql.push_str("password = $password, ");
            Value::from(p)
        },
        None => Value::None,
    };

    // Convert the id to a surrealdb::sql::value
    // This means I dont have to case anything in the SQL
    // I dont have to explicitly do this but I prefer to
    let id: Value = Thing::from(("User", id)).into();

    // Remove the end space and end comma and add the return statement
    sql.pop();
    sql.pop();
    sql.push_str(" RETURN AFTER;");

    // Send the query
    let mut response = DB.query(sql)
        .bind(("id", id))
        .bind(("email", email))
        .bind(("username", username))
        .bind(("password", password))
        .await
        .unwrap(); // This will only panic if the sql is malformed or there is a critical DB error

    let result: Option<User> = response
    .take(0)
    .map_err(|e| {
        DBEditError::Other(e.to_string())
    })?;

    let result = result.ok_or_else(|| {
        DBEditError::NotFound("Failed to get task".to_string())
    })?;
    
    Ok(result)
}

#[allow(dead_code)]
/// Delete a user from the database
/// 
/// # Arguments
/// * `id` - The id of the user to delete
/// 
/// # Returns
/// `Result<User, DBEditError>` - The deleted user or an error
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