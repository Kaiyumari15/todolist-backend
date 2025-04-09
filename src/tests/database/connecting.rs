#[cfg(test)]
mod connecting {
    use crate::database::{connect, clear_all_test};

    #[tokio::test]
    /// Test the database connection function does not panic
    /// We do not test any data in the database, this is done in the respective modules
    async fn test_connect() {
        let _ = connect().await;
    }

    #[tokio::test]
    /// Test the database can correctly clear all test data
    /// We do not test any data in the database, this is done in the respective modules
    async fn test_clear_all_test() {
        let _ = connect().await;
        let _ = clear_all_test().await;
    }

}