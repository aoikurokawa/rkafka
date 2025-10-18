use crate::{body::Body, header::ResponseHeader};

pub struct Response<B: Body> {
    /// The message_size field gives the size of the subsequent request or response message in bytes.
    /// The client can read requests by first reading this 4 byte size as an integer N, and then reading and parsing the subsequent N bytes of the request.
    message_size: u32,

    /// Header
    header: ResponseHeader,

    /// Body
    body: B,
}

impl<B: Body> Response<B> {
    pub fn new(correlation_id: i32, body: B) -> Self {
        let header = ResponseHeader::new(correlation_id);

        let message_size = header.size() + body.size();

        Self {
            message_size,
            header,
            body,
        }
    }

    /// Serialize the response into bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.message_size.to_be_bytes());
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.body.to_bytes());

        bytes
    }
}
