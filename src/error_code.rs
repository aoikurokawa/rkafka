#[derive(Debug, Clone, Copy)]
#[repr(i16)]
pub enum ErrorCode {
    UnknownServerError = -1,

    NONE = 0,

    UnsupportedVersion = 35,
}
