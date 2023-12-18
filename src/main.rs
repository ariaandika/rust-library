#![allow(warnings)]

use std::time::Duration;

use tokio::net::*;

pub mod async_in_sync;
pub mod tokio_spawn;
pub mod tcp_listener;

#[tokio::main]
async fn main() {
    // async_in_sync::async_in_sync();
    // tokio_spawn::tokio_spawn();
    tcp_listener::tcp_listener().await;
}

