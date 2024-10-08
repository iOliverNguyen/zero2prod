#[cfg(test)]
mod tests;

use std::net::TcpListener;

use newsletter::run;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let _ = run(listener)?.await;
    Ok(())
}
