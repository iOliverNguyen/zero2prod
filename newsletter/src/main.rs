#[cfg(test)]
mod tests;

use newsletter::run;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
