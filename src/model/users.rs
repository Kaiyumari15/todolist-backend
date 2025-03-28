use surrealdb::sql::Thing;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// This represents a user of the application / an account
pub struct User {
    id: Option<Thing>,
    username: Option<String>,
    email: Option<String>,
    passsword: Option<String>
}