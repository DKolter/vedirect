use super::HexRecordError;

#[derive(Debug)]
pub enum ErrorType {
    FrameError,
    BootloaderFailed,
}

impl ErrorType {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HexRecordError> {
        match bytes {
            [10, 10, 10, 10] => Ok(Self::FrameError),
            [0, 0, 0, 0] => Ok(Self::BootloaderFailed),
            _ => Err(HexRecordError::WrongFormat),
        }
    }
}
