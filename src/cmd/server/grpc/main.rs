#![allow(non_snake_case)]

#[path = "api/api.rs"]
pub(crate) mod api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  Ok(())
}

async fn run() {}
