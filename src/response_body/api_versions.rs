use crate::{api_key::ApiKey, body::Body, error_code::ErrorCode};

#[derive(Debug, Clone, Copy)]
pub struct ApiSupport {
    /// The API index
    pub api_key: ApiKey,

    /// The minimum supported version, inclusive.
    pub min_version: i16,

    /// The maximum supported version, inclusive
    pub max_version: i16,

    /// Tag buffer
    pub tag_buffer: u8,
}

#[derive(Debug)]
pub struct ApiVersionsResponse {
    /// The top-level error code
    pub error_code: ErrorCode,

    /// The APIs supported by the broker
    pub api_keys: Vec<ApiSupport>,

    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    pub throttle_time_ms: i32,

    /// Tag buffer
    pub tag_buffer: u8,
}

impl Body for ApiVersionsResponse {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let error_code = self.error_code as i16;
        bytes.extend_from_slice(&error_code.to_be_bytes());

        let array_len = (self.api_keys.len() + 1) as u8;
        bytes.push(array_len);

        for api in self.api_keys.iter() {
            let api_key = api.api_key as i16;
            bytes.extend_from_slice(&api_key.to_be_bytes());

            bytes.extend_from_slice(&api.min_version.to_be_bytes());
            bytes.extend_from_slice(&api.max_version.to_be_bytes());
            bytes.extend_from_slice(&api.tag_buffer.to_be_bytes());
        }

        bytes.extend_from_slice(&self.throttle_time_ms.to_be_bytes());
        bytes.extend_from_slice(&self.tag_buffer.to_be_bytes());

        bytes
    }

    fn size(&self) -> u32 {
        let error_code = self.error_code as i16;
        let error_code_len = error_code.to_be_bytes().len() as u32;

        let array_len_prefix = 1;

        let api_keys_len: u32 = self
            .api_keys
            .iter()
            .map(|key| {
                let api_key = key.api_key as i16;
                let api_key_len = api_key.to_be_bytes().len() as u32;

                let min_version_len = key.min_version.to_be_bytes().len() as u32;
                let max_version_len = key.max_version.to_be_bytes().len() as u32;
                let tag_buffer_len = key.tag_buffer.to_be_bytes().len() as u32;

                api_key_len + min_version_len + max_version_len + tag_buffer_len
            })
            .sum();

        let throttle_time_ms_len = self.throttle_time_ms.to_be_bytes().len() as u32;
        let tag_buffer_len = self.tag_buffer.to_be_bytes().len() as u32;

        error_code_len + array_len_prefix + api_keys_len + throttle_time_ms_len + tag_buffer_len
    }
}

impl ApiVersionsResponse {
    pub fn new(error_code: ErrorCode) -> Self {
        Self {
            error_code,
            api_keys: vec![
                ApiSupport {
                    api_key: ApiKey::ApiVersions,
                    min_version: 0,
                    max_version: 4,
                    tag_buffer: 0,
                },
                ApiSupport {
                    api_key: ApiKey::DescribeTopicPartitions,
                    min_version: 0,
                    max_version: 4,
                    tag_buffer: 0,
                },
            ],
            throttle_time_ms: 0,
            tag_buffer: 0,
        }
    }
}
