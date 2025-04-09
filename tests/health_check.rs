use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));

}

fn spawn_app() -> String {
    let address = "127.0.0.1:0";
    let listener =
        TcpListener::bind(address).expect(&format!("Failed to bind to {}", address));
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to start zero2prod");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}