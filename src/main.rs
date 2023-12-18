#![allow(warnings)]

use std::time::Duration;

use tokio::net::*;



// #[tokio::main]
fn main() {
    async_in_sync();
    tcp_listener();
}

fn tcp_listener() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let handle = tokio::spawn(async {
                process(socket).await
            });

            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("socketing");

            println!("After socket {:?}", handle.await);
        }
    });
}

async fn process(socket: TcpStream) -> i32 {
    println!("Processing socket");
    443
}

async fn async_block_capture() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let v = vec![1,2,3];

        tokio::spawn(async move {
            println!("{:?}",v);
        })
    });
}

fn async_in_sync() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let res = rt.block_on(async {
        app().await
    });

    println!("oof done")
}

async fn app() -> i32 {
    println!("oof");
    tokio::time::sleep(Duration::from_secs(1)).await;
    443
}
