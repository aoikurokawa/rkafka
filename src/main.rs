use anyhow::anyhow;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

use rkafka::{api_key::ApiKey, request_header::RequestHeader};

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
                let message_size = u32::from_be_bytes(size_buf);

                let mut message_buf = vec![0; message_size as usize];
                if let Err(e) = stream.read_exact(&mut message_buf) {
                    println!("Error reading message: {e}");
                    continue;
                }

                let api_key = ApiKey::try_from([message_buf[0], message_buf[1]].as_slice())
                    .map_err(|e| anyhow!("{e}"))?;
                let request_api_version = i16::from_be_bytes([message_buf[2], message_buf[3]]);
                let correlation_id = i32::from_be_bytes([
                    message_buf[4],
                    message_buf[5],
                    message_buf[6],
                    message_buf[7],
                ]);

                let request_header =
                    RequestHeader::new(api_key, request_api_version, correlation_id);

                let mut body_buf = Vec::new();
                let _size = stream.read_to_end(&mut body_buf)?;

                // if matches!(request_header.request_api_key, ApiKey::ApiVersions) {
                stream.write_all(&0i32.to_be_bytes())?;
                stream.write_all(&correlation_id.to_be_bytes())?;
                stream.write_all(&[0, 35])?;
                // }

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
