use std::str;

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use std::io::prelude::*;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

static PROXY_LOCALHOST_ADDRESS: &str = "127.0.0.1:34372";
pub const BUFFER_SIZE: usize = 1024;

pub async fn search(text: String) -> Result<crate::api::SearchResponse, String> {
    log::info!("Send search request for `{}`", text);
    let mut request: crate::api::Request = crate::api::Request::default();
    request.op = crate::api::OperationType::Search;
    request.payload = match serde_json::to_string(&text) {
        Ok(p) => p.as_bytes().to_vec(),
        Err(e) => return Err(e.to_string()),
    };
    let response = match send_request::<crate::api::SearchResponse>(request).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    Ok(response)
}

pub async fn ping() -> Result<crate::api::PingResponse, String> {
    log::info!("Send ping request");
    let mut request: crate::api::Request = crate::api::Request::default();
    request.op = crate::api::OperationType::Ping;
    let response = match send_request::<crate::api::PingResponse>(request).await {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    Ok(response)
}

pub async fn send_request<T>(request: crate::api::Request) -> Result<T, String>
where
    T: DeserializeOwned,
{
    // TODO also consider https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect_timeout
    let mut stream = match TcpStream::connect(PROXY_LOCALHOST_ADDRESS).await {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    // TODO see https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout
    // TODO see https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_write_timeout
    stream
        .write_all(&request.to_bytes())
        .await
        .map_err(|e| e.to_string());

    // FIXME we should have a bigger buffer here, no?
    let mut buf = vec![0u8; BUFFER_SIZE];
    let n = stream.read(&mut buf).await.map_err(|e| e.to_string());

    let response = match crate::api::Response::from_bytes(&buf) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    let response: T = match serde_json::from_str(str::from_utf8(&response.payload).unwrap()) {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    Ok(response)
}

pub async fn start_server() -> Result<(), String> {
    // TODO make the port parameterable.
    let listener = match TcpListener::bind(PROXY_LOCALHOST_ADDRESS).await {
        Ok(l) => l,
        Err(e) => return Err(e.to_string()),
    };

    while true {
        for stream in listener.incoming().next().await {
            let mut stream = stream.unwrap();

            log::debug!("TCP connection opened from {}", stream.peer_addr().unwrap());

            // TODO call stream.read_to_string() instead?
            let mut buffer = [0; BUFFER_SIZE];
            stream.read(&mut buffer).await.unwrap();

            let request = match crate::api::Request::from_bytes(&buffer) {
                Ok(r) => r,
                Err(e) => return Err(format!("Could not parse request from client: {}", e)),
            };

            log::info!("Received request {:?}", request.op);

            let response = crate::api::Response::default();
            // TODO call handle_request
            stream.write(&response.to_bytes()).await.unwrap();
            stream.flush().await.unwrap();
        }
    }
    Ok(())
}
