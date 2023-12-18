use std::{io, sync::Arc};

use tokio::{net::{TcpListener, tcp::ReadHalf}, io::*};

pub async fn tcp_listener() -> io::Result<()> {
    let tcp = TcpListener::bind("127.0.0.1:3000").await?;

    let len = tokio::fs::metadata("readme.md").await.unwrap().len();

    loop {
        let (mut stream, addr) = tcp.accept().await?;

        tokio::spawn(async move {
            let (mut req_reader,mut req_writer) = stream.split();

            let headers = manual_read(&mut req_reader).await;
            println!("Headers: {:#?}", headers);

            req_writer.write_all(
                format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 11\r\nConnection: keep-alive\r\nKeep-Alive: 10\r\n\r\n").as_bytes()
            ).await.unwrap();

            // tokio::io::copy(&mut file_reader, &mut req_writer).await.unwrap();
            req_writer.write_all(b"Hello world").await.unwrap();

            manual_read(&mut req_reader).await;
        });

    }
}

async fn read_request(mut reader: ReadHalf<'_>) {
    let mut buf = [0;100];
    let app = reader.read(&mut buf).await.unwrap();
    println!("Req: {:?}",String::from_utf8(buf.to_vec()));
}

async fn manual_read(mut socket: &mut ReadHalf<'_>) -> Vec<String> {
    let mut buf = [0;100]; // vec![0; 10];
    let mut rs_temp = String::new();
    let mut result = vec![];
    let mut reader = tokio::io::BufReader::new(&mut socket);
    loop {
        match reader.read_line(&mut rs_temp).await {
            Ok(0) => break,
            Ok(2) => break,
            Ok(n) => {
                result.push(rs_temp.clone());
                rs_temp.clear();
            }
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                break;;
            }
        }
    }

    result
}
