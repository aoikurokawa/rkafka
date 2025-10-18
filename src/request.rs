use anyhow::anyhow;

use crate::{api_key::ApiKey, header::Header};

#[derive(Debug)]
pub struct Request {
    /// The message_size field gives the size of the subsequent request or response message in bytes.
    /// The client can read requests by first reading this 4 byte size as an integer N, and then reading and parsing the subsequent N bytes of the request.
    pub message_size: u32,

    /// Header
    pub header: Header,
    // Body
}

// impl TryFrom<&[u8]> for RequestMessage {
//     type Error = String;
//
//     fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
//         if value.len() < 4 + 2 + 2 + 4 {
//             return Err("Size constraints not satisfied".to_string());
//         }
//         let message_size = i32::from_be_bytes(value[..4].try_into().unwrap());
//         let header = RequestHeader::try_from(&value[4..12]).unwrap();
//         // let body = String::from_utf8(value[12..].to_vec()).unwrap();
//
//         Ok(Self {
//             message_size,
//             header,
//             //   body,
//         })
//     }
// }

impl Request {
    pub fn new(message_size: u32, message_buf: &[u8]) -> anyhow::Result<Self> {
        let api_key = ApiKey::try_from([message_buf[0], message_buf[1]].as_slice())
            .map_err(|e| anyhow!("{e}"))?;
        let request_api_version = i16::from_be_bytes([message_buf[2], message_buf[3]]);
        let correlation_id = i32::from_be_bytes([
            message_buf[4],
            message_buf[5],
            message_buf[6],
            message_buf[7],
        ]);

        let header = Header::new(api_key, request_api_version, correlation_id);

        Ok(Self {
            message_size,
            header,
        })
    }
}
