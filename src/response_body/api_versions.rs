use crate::{body::Body, error_code::ErrorCode};

#[derive(Debug)]
pub struct ApiVersionsResponse {
    pub error_code: ErrorCode,
}

impl Body for ApiVersionsResponse {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let error_code = self.error_code as i16;
        bytes.extend_from_slice(&error_code.to_be_bytes());

        bytes
    }
}

impl ApiVersionsResponse {
    pub fn new(error_code: ErrorCode) -> Self {
        Self { error_code }
    }
}
