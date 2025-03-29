use surrealdb::sql::Thing;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// This represents a user of the application / an account
pub struct User {
    pub id: Option<Thing>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<String>,
}