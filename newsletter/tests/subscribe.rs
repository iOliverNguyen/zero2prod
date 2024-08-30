use reqwest::{header, multipart};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Serialize)]
struct FormData {
    name: String,
    email: String,
}

#[derive(Deserialize)]
struct SubscribeResponse {
    name: String,
    email: String,
    status: String,
}

#[derive(Deserialize)]
struct SubscribeError {
    error: String,
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let addr = spawn_app();

    let form = FormData {
        name: "Oliver Nguyen".to_string(),
        email: "oliver@example.com".to_string(),
    };

    let client = reqwest::Client::new();
    let resp = client
        .post(&format!("{}/subscribe", addr))
        .form(&form)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(
        resp.status().as_u16(),
        200,
        "expected 200, got {}",
        resp.status()
    );

    let resp_body: SubscribeResponse = resp.json().await.expect("Failed to parse JSON");
    assert_eq!(
        resp_body.name, "Oliver Nguyen",
        "expected name to be 'Oliver Nguyen', got '{}'",
        resp_body.name
    );
    assert_eq!(
        resp_body.email, "oliver@example.com",
        "expected email to be 'oliver@example.com', got '{}'",
        resp_body.email
    );
    assert_eq!(
        resp_body.status, "ok",
        "expected status to be 'ok', got '{}'",
        resp_body.status
    );
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    struct TestCase {
        name: String,
        email: String,
        expected_error: String,
    }

    let addr = spawn_app();
    let client = reqwest::Client::new();
    let testcases = vec![
        TestCase {
            name: "".to_string(),
            email: "oliver@example.com".to_string(),
            expected_error: "missing name".to_string(),
        },
        TestCase {
            name: "Oliver Nguyen".to_string(),
            email: "".to_string(),
            expected_error: "missing email".to_string(),
        },
        TestCase {
            name: "".to_string(),
            email: "".to_string(),
            expected_error: "missing name and email".to_string(),
        },
    ];

    for tc in testcases {
        let form = FormData {
            name: tc.name,
            email: tc.email,
        };

        let resp = client
            .post(&format!("{}/subscribe", addr))
            .form(&form)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(
            resp.status().as_u16(),
            400,
            "expected 400, got {}",
            resp.status()
        );

        // Parse the response body as JSON
        let resp_body: SubscribeError = resp.json().await.expect("Failed to parse JSON");
        assert_eq!(
            resp_body.error, tc.expected_error,
            "expected error message '{}', got '{}'",
            tc.expected_error, resp_body.error
        );
    }
}

pub fn spawn_app() -> String {
    // Spawn the app on a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind with random port");
    let port = listener.local_addr().unwrap().port();

    // Start the server as background task
    let server = newsletter::run(listener).expect("Failed to run newsletter");
    let _ = tokio::spawn(server);

    // Return the address
    format!("http://127.0.0.1:{}", port)
}
