use std::{
    io::{Read, Write},
    net::TcpListener,
};

struct Message {
    // message_size: u32,
    request_api_key: i16,
    request_api_version: i16,
    correlation_id: i32,
    client_id: Option<String>,
    tag_buffer: Vec<u8>,
}

impl Message {
    fn new(
        //   message_size: u32,
        request_api_key: i16,
        request_api_version: i16,
        correlation_id: i32,
    ) -> Self {
        Self {
            //    message_size,
            request_api_key,
            request_api_version,
            correlation_id,
            client_id: None,
            tag_buffer: vec![],
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // let mut buf = vec![0; 100];
                // let size = stream.read_to_end(&mut buf).unwrap();

                // println!("Size: {size}");

                let mut size_buf = [0; 4];
                if let Err(e) = stream.read_exact(&mut size_buf) {
                    println!("Error reading message size: {e}");
                    continue;
                }
                let message_size = u32::from_be_bytes(size_buf);
                println!("Message size: {message_size}");

                let mut message_buf = vec![0; message_size as usize];
                if let Err(e) = stream.read_exact(&mut message_buf) {
                    println!("Error reading message: {e}");
                    continue;
                }

                // request_api_key_buf.copy_from_slice(&buf[4..6]);
                let request_api_key = i16::from_be_bytes([message_buf[0], message_buf[1]]);

                // let mut request_api_version_buf = [0; 2];
                // request_api_version_buf.copy_from_slice(&buf[6..8]);
                let request_api_version = i16::from_be_bytes([message_buf[2], message_buf[3]]);

                // let mut correlation_id_buf = [0; 4];
                // correlation_id_buf.copy_from_slice(&buf[8..12]);
                let correlation_id = i32::from_be_bytes([
                    message_buf[4],
                    message_buf[5],
                    message_buf[6],
                    message_buf[7],
                ]);

                println!("Correlation ID: {correlation_id}");

                stream.write_all(&mut correlation_id.to_be_bytes()).unwrap();

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
