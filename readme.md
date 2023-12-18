# Tokio

async await in rust

titles:

- async in sync function
- tokio::spawn
- async block capture
- sharing state, mutex
- sharing state, channel
- io streams

> bytes dependency is equal to `Arc<Vec<u8>>`, when cloned, its not cloning underlying data

tokio async is lazy, will not run, unless await is called

to run async concurently:

- tokio::spawn, run this concurently
- select!, which one resolve first
- join!, await all concurently
- mpsc::channel

## async in sync function

use async expression

dont use this in async main function,
nested runtime is bad says tokio

```rust
let mut rt = tokio::runtime::Runtime::new().unwrap();
rt.block_on(async {
    println!("hello");
})
```

## tokio::spawn

A Tokio task is an asynchronous green thread

`tokio::spawn` is spawning task, runned eagerly

```rust
// this eagerly runned
let handle = tokio::spawn(async {
    let res: i32 = process(socket).await;
    res
});

let res: i32 = handle.await;
```

## async block capture

```rust
let v = vec![1,2,3];

tokio::spawn(async {
    println!("{:?}",v);
});
```

## sharing state, mutex

```rust
use std::sync::Mutex;

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

let db = Arc::new(Mutex::new(HashMap::new()));

loop {
    // Clone the handle to the hash map.
    let db = db.clone();

    tokio::spawn(async move {
        process(db).await;
    });
}
```

## sharing state, channel

- mpsc: multi-producer, single-consumer channel.
- oneshot: single-producer, single consumer channel.
- broadcast: multi-producer, multi-consumer.
- watch: single-producer, multi-consumer.

> async-channel crate, for multi-producer multi-consumer, with one to one producer and consumer

```rust
use tokio::sync::mpsc;

// Create a new channel with a capacity of at most 32.
let (tx, mut rx) = mpsc::channel(32);

// clone for each async block
let tx2 = tx.clone();

tokio::spawn(async move {
    tx.send("sending from first handle").await;
});

tokio::spawn(async move {
    tx2.send("sending from second handle").await;
});

while let Some(message) = rx.recv().await {
    println!("GOT = {}", message);
}
```

## io stream

tokio io utility

- io::copy, read to writer
- io::split, split socket to reader and writer

```rust
use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4040").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut rd, mut wr) = socket.split();
    
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

async fn client() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:4040").await?;
    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;
        // Sometimes, the rust type inferencer needs
        // a little help
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;
        if n == 0 { break; }
        println!("GOT {:?}", &buf[..n]);
    }

    Ok(())
}
```
