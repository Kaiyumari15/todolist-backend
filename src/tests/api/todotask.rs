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

#[cfg(test)]
mod editing_tasks {
    use chrono::{DurationRound, TimeDelta, Utc};
    use rocket::{http::{hyper::header::CONTENT_TYPE, ContentType, Status}, local::asynchronous::Client, routes};

    use crate::{api::todotask::{create_task_handler, update_task_handler}, database::connect, model::todotask::ToDoTask};


    #[tokio::test]
    async fn test_update_task_handler_success() {
        // Set up the database connection and rocket instandce
        let _ = connect().await;
        let client = Client::tracked((rocket::build().mount("/", routes![create_task_handler, update_task_handler]))).await.unwrap();

        // Create a task to edit 
        let original_task = ToDoTask {
            id: None,
            title: Some("TESToldtitle".to_string()),
            description: Some("TESTolddescription".to_string()),
            completed_at: None,
            created_at: None,
        };
        let create_response = client.post("/tasks").header(ContentType::JSON).body(serde_json::to_string(&original_task).unwrap()).dispatch().await;
        assert_eq!(create_response.status(), Status::Created, "Error creating task to be edited {}", create_response.status());
        let old_task: ToDoTask = create_response.into_json().await.unwrap();


        // Edit the task
        let now_str = Utc::now()
            .duration_round(
                TimeDelta::try_milliseconds(10)
                .unwrap())
            .unwrap()
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let updated_task = ToDoTask {
            id: None,
            title: Some("TESTnewtitle".to_string()),
            description: Some("TESTnewdescription".to_string()),
            completed_at: Some(now_str),
            created_at: None,
        };
        let edit_response = client.patch(format!("/tasks/{}", old_task.id.unwrap().id.to_string())).header(ContentType::JSON).body(serde_json::to_string(&updated_task).unwrap()).dispatch().await;
        assert_eq!(edit_response.status(), Status::Ok, "Error when updating the task: {}: {}", edit_response.status(), edit_response.into_string().await.unwrap());
    }
}

mod deleting_tasks {
    use rocket::{http::{ContentType, Status}, local::asynchronous::Client, response, routes};

    use crate::{api::todotask::{create_task_handler, delete_task_handler}, database::connect, model::todotask::ToDoTask};

    #[tokio::test]
    async fn test_delete_task_handler_success() {
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