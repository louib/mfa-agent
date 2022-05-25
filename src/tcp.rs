use std::net::TcpListener;

async fn send_data() -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
    Ok(())
}

async fn start_server() -> Result<(), String> {
    Ok(())
}
