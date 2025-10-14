use crate::request_header::RequestHeader;

#[derive(Debug)]
struct RequestMessage {
    /// Message size
    message_size: i32,

    /// Header
    header: RequestHeader,
    // body
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
