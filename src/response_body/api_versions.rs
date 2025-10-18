use crate::{api_key::ApiKey, body::Body, error_code::ErrorCode};

#[derive(Debug)]
pub struct ApiSupport {
    /// The API index
    pub api_key: ApiKey,

    /// The minimum supported version, inclusive.
    pub min_version: i16,

    /// The maximum supported version, inclusive
    pub max_version: i16,
}

#[derive(Debug)]
pub struct ApiVersionsResponse {
    /// The top-level error code
    pub error_code: ErrorCode,

    /// The APIs supported by the broker
    pub api_keys: Vec<ApiSupport>,
}

impl Body for ApiVersionsResponse {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let error_code = self.error_code as i16;
        bytes.extend_from_slice(&error_code.to_be_bytes());

        bytes
    }

    fn size(&self) -> u32 {
        let error_code = self.error_code as i16;
        error_code.to_be_bytes().len() as u32
    }
}

impl ApiVersionsResponse {
    pub fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            api_keys: vec![ApiSupport {
                api_key: ApiKey::ApiVersions,
                min_version: 0,
                max_version: 4,
            }],
        }
    }
}
