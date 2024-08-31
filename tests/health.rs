use std::net::TcpListener;

use reqwest;

#[tokio::test]
async fn health_test_ok() {
    let addr = spawn_app("127.0.0.1:0");

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    // test the response body
    let body = response.text().await.expect("Failed to read response body");
    assert_eq!(body, r#"{"status":"ok"}"#);
}

fn spawn_app(addr: &str) -> String {
    // Spawn the app on a random port
    let listener = TcpListener::bind(addr).expect("Failed to bind with random port");
    let port = listener.local_addr().unwrap().port();

    // Start the server as background task
    let server = newsletter::run(listener).expect("Failed to run newsletter");
    let _ = tokio::spawn(server);

    // Return the address
    format!("127.0.0.1:{}", port)
}
