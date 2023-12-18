use std::time::Duration;




pub fn tokio_spawn() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        run().await
    });
}

async fn run() {
    let handle = tokio::spawn(async {
        println!("Shout from inside async block");
        445
    });

    println!("Await from outside async...");
    std::thread::sleep(Duration::from_secs(3));
    let res = handle.await;

    println!("{:?}",res);
}
