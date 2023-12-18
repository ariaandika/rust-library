use std::time::Duration;


pub fn async_in_sync() {
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
