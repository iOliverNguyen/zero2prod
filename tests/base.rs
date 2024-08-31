use std::net::TcpListener;

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
