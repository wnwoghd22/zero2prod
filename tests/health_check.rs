//! tests/health_check.rs

#[tokio::test]
async fn dummy_test() {
    // Arrange
    spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    // todo!()
    let server = zero2prod::run().expect("failed to bind address");

    let _ = tokio::spawn(server);
}