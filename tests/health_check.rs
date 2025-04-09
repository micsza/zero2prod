#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://localhost:8080/health_check")
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));

}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to start zero2prod");

    let _ = tokio::spawn(server);
}