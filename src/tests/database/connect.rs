#[cfg(test)]
mod connect {
    use crate::database::{connect, create_all};

    #[tokio::test]
    async fn test_connect_success() {
        // Assuming the database is running
        connect().await;
        // If no panic occurred, the connection was successful
    }

    #[tokio::test]
    async fn test_create_all_success() {
        // Assuming the database is running
        connect().await;
        create_all().await;
        // If no panic occurred, the table and fields were created successfully
    }
}