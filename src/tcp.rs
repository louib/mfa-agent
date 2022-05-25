use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;

static PROXY_LOCALHOST_ADDRESS: &str = "127.0.0.1:34372";

pub async fn send_data(data: Vec<u8>) -> Result<(), String> {
    // TODO also consider https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect_timeout
    let mut stream = match TcpStream::connect(PROXY_LOCALHOST_ADDRESS).await {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    stream.write_all(&data).await.map_err(|e| e.to_string());

    // FIXME we should have a bigger buffer here, no?
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
        let mut stream = stream.unwrap();

        log::debug!("TCP connection opened from {}", stream.peer_addr().unwrap());

        // TODO call stream.read_to_string() instead?
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).await.unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
    Ok(())
}
