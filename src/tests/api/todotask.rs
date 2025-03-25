

#[cfg(test)]
mod creating_tasks {
    use crate::model::todotask::ToDoTask;
    use crate::api::todotask::create_task_handler;
    use crate::database::connect;
    use rocket::http::{Status, ContentType};
    use rocket::local::asynchronous::Client;
    use rocket::routes;
    use rocket::serde::json::Json;

    #[tokio::test]
    async fn test_create_task_handler_success() {
        let _ = connect().await;
        let client = Client::tracked(rocket::build().mount("/", routes![create_task_handler])).await.unwrap();
        let task = ToDoTask {
            id: None,
            title: Some("Test Task".to_string()),
            description: Some("This is a test task".to_string()),
            completed_at: None,
            created_at: None,
        };
        let response = client.post("/tasks").header(ContentType::JSON).body(serde_json::to_string(&task).unwrap()).dispatch().await;
        assert_eq!(response.status(), Status::Created);
    }
}