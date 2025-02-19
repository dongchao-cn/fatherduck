mod connection;
mod server;
mod auth;
mod query;
mod error;
mod parser;
mod config;

use server::start_server;

#[tokio::main]
pub async fn main() {
    start_server().await
}
