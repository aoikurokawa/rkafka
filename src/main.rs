use std::{
    io::{Read, Write},
    net::TcpListener,
};

use rkafka::{
    error_code::ErrorCode, request::Request, response::Response,
    response_body::api_versions::ApiVersionsResponse,
};

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut size_buf = [0; 4];
                if let Err(e) = stream.read_exact(&mut size_buf) {
                    println!("Error reading message size: {e}");
                    continue;
                }
                let request_message_size = u32::from_be_bytes(size_buf);

                let mut message_buf = vec![0; request_message_size as usize];
                if let Err(e) = stream.read_exact(&mut message_buf) {
                    println!("Error reading message: {e}");
                    continue;
                }

                let request = Request::new(request_message_size, message_buf.as_slice())?;

                let api_versions_response = ApiVersionsResponse::new(ErrorCode::NONE);
                let response = Response::new(request.header.correlation_id, api_versions_response);

                let response_bytes = response.to_bytes();
                stream.write_all(&response_bytes)?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
