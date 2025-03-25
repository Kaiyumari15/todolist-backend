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

mod deleting_tasks {
    use rocket::{http::{ContentType, Status}, local::asynchronous::Client, response, routes};

    use crate::{api::todotask::{create_task_handler, delete_task_handler}, database::connect, model::todotask::ToDoTask};

    #[tokio::test]
    pub async fn test_delete_task_handler_success() {
        // set up the database connection and rocket instance
        let _ = connect().await;
        let client = Client::tracked(rocket::build().mount("/", routes![create_task_handler, delete_task_handler])).await.unwrap();

        // create a task to delete
        let task = ToDoTask {
            id: None,
            title: Some("Test Task".to_string()),
            description: Some("This is a test task".to_string()),
            completed_at: None,
            created_at: None,
        };
        let response = client.post("/tasks").header(ContentType::JSON).body(serde_json::to_string(&task).unwrap()).dispatch().await;
        assert_eq!(response.status(), Status::Created);
        let created_task: ToDoTask = response.into_json().await.unwrap(); // get the created task from the response

        // delete the task
        let response = client.delete(format!("/tasks/{}", created_task.id.unwrap().id.to_string())).dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        let deleted_task: ToDoTask = response.into_json().await.unwrap(); // get the deleted task from the response
    }
}