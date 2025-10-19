use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use rkafka::{
    api_key::ApiKey, error_code::ErrorCode, request::Request, response::Response,
    response_body::api_versions::ApiVersionsResponse,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        println!("Connection error: {e}");
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    loop {
        // Read message size (4 bytes)
        let mut size_buf = [0; 4];
        match stream.read_exact(&mut size_buf) {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                println!("Connection closed");
                break;
            }
            Err(e) => return Err(e.into()),
        }

        let request_message_size = u32::from_be_bytes(size_buf);

        // Read the message body
        let mut message_buf = vec![0; request_message_size as usize];
        stream.read_exact(&mut message_buf)?;

        // Parse the request
        let request = Request::new(request_message_size, message_buf.as_slice())?;

        println!(
            "Received {:?} request (correlation_id: {})",
            request.header.api_key, request.header.correlation_id
        );

        // Handle different API keys

        if let ApiKey::ApiVersions = request.header.api_key {
            let response = if request.header.api_version <= 4 {
                let api_versions_response = ApiVersionsResponse::new(ErrorCode::NONE);
                Response::new(request.header.correlation_id, api_versions_response)
            } else {
                let api_versions_response = ApiVersionsResponse::new(ErrorCode::UnsupportedVersion);
                Response::new(request.header.correlation_id, api_versions_response)
            };

            // Send response
            stream.write_all(&response.to_bytes())?;
            stream.flush()?;
        }

        println!("Response sent");
    }

    Ok(())
}
