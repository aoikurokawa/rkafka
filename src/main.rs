use std::{
    io::{Read, Write},
    net::TcpListener,
};

#[derive(Debug)]
struct RequestHeaderV2 {
    /// Request API key
    request_api_key: i16,

    /// Request API version
    request_api_version: i16,

    /// Correlation ID
    correlation_id: i32,
}

impl TryFrom<&[u8]> for RequestHeaderV2 {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 2 + 2 + 4 {
            return Err("Size constraints not satisfied".to_string());
        }

        Ok(Self {
            request_api_key: i16::from_be_bytes(value[0..2].try_into().unwrap()),
            request_api_version: i16::from_be_bytes(value[2..4].try_into().unwrap()),
            correlation_id: i32::from_be_bytes(value[4..8].try_into().unwrap()),
        })
    }
}

#[derive(Debug)]
struct RequestMessage {
    message_size: i32,
    header: RequestHeaderV2,
    // body
}

impl TryFrom<&[u8]> for RequestMessage {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 4 + 2 + 2 + 4 {
            return Err("Size constraints not satisfied".to_string());
        }
        let message_size = i32::from_be_bytes(value[..4].try_into().unwrap());
        let header = RequestHeaderV2::try_from(&value[4..12]).unwrap();
        // let body = String::from_utf8(value[12..].to_vec()).unwrap();

        Ok(Self {
            message_size,
            header,
            //   body,
        })
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
                let mut size_buf = [0; 4];
                // let n = stream.read(buf)
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

                let request_api_key = i16::from_be_bytes([message_buf[0], message_buf[1]]);
                let request_api_version = i16::from_be_bytes([message_buf[2], message_buf[3]]);
                let correlation_id = i32::from_be_bytes([
                    message_buf[4],
                    message_buf[5],
                    message_buf[6],
                    message_buf[7],
                ]);

                stream
                    .write_all(&mut [
                        0,
                        0,
                        0,
                        0,
                        message_buf[4],
                        message_buf[5],
                        message_buf[6],
                        message_buf[7],
                    ])
                    .unwrap();

                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
