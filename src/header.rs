use crate::api_key::ApiKey;

#[derive(Debug)]
pub struct Header {
    /// Request API key
    pub api_key: ApiKey,

    /// Request API version
    pub api_version: i16,

    /// Correlation ID
    pub correlation_id: i32,
}

impl Header {
    pub fn new(api_key: ApiKey, api_version: i16, correlation_id: i32) -> Self {
        Self {
            api_key,
            api_version,
            correlation_id,
        }
    }

    /// Serialize the response into bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let api_key_value = self.api_key as i16;
        bytes.extend_from_slice(&api_key_value.to_be_bytes());
        bytes.extend_from_slice(&self.api_version.to_be_bytes());
        bytes.extend_from_slice(&self.correlation_id.to_be_bytes());

        bytes
    }
}
