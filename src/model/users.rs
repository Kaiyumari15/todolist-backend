use surrealdb::sql::Thing;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// This represents a user of the application / an account of the app
/// 
/// # Fields
/// * `id` - The ID of the user
/// * `username` - The username of the user
/// * `email` - The email of the user
/// * `password` - The password of the user
/// * `created_at` - The date and time when the user was created
pub struct User {
    pub id: Option<Thing>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub created_at: Option<String>,
}