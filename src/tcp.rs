use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;

static PROXY_LOCALHOST_ADDRESS: &str = "127.0.0.1:34372";

async fn send_data(data: Vec<u8>) -> Result<(), String> {
    // TODO also consider https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect_timeout
    let mut stream = match TcpStream::connect(PROXY_LOCALHOST_ADDRESS).await {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    stream.write_all(b"hello world").await.map_err(|e| e.to_string());

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await.map_err(|e| e.to_string());

    // TODO see https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout
    // TODO see https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_write_timeout

    Ok(())
}

pub async fn start_server() -> Result<(), String> {
    // TODO make the port parameterable.
    let listener = match TcpListener::bind(PROXY_LOCALHOST_ADDRESS).await {
        Ok(l) => l,
        Err(e) => return Err(e.to_string()),
    };

    for stream in listener.incoming().next().await {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
    Ok(())
}
