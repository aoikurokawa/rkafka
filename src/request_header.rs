use crate::api_key::ApiKey;

#[derive(Debug)]
pub struct RequestHeader {
    /// Request API key
    pub request_api_key: ApiKey,

    /// Request API version
    pub request_api_version: i16,

    /// Correlation ID
    pub correlation_id: i32,
}

impl RequestHeader {
    pub fn new(request_api_key: ApiKey, request_api_version: i16, correlation_id: i32) -> Self {
        Self {
            request_api_key,
            request_api_version,
            correlation_id,
        }
    }
}
