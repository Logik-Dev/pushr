pub use error::{Error, Result};

mod cli;
mod error;
mod http;

#[tokio::main]
async fn main() -> Result<()> {
    http::send(cli::parse()?).await
}
