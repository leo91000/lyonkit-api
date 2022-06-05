mod config;
mod errors;
mod middlewares;
mod server;
mod services;
mod telemetry;

use crate::{config::SETTINGS, server::Server, telemetry::init_tracing};
use std::io;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() -> io::Result<()> {
  init_tracing();

  let server = Server::from_settings(&*SETTINGS)
    .await
    .migrate()
    .await
    .build()?;

  server.run_until_stopped().await?;

  Ok(())
}
