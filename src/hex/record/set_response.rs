use super::HexRecordError;

#[derive(Debug)]
pub enum SetResponse {
    Ok,
    UnknownRegister,
    NotSupported,
    ParameterError,
}

impl SetResponse {
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, HexRecordError> {
        match buffer {
            [_, _, flags, ..] => match flags {
                0 => Ok(Self::Ok),
                1 => Ok(Self::UnknownRegister),
                2 => Ok(Self::NotSupported),
                3 => Ok(Self::ParameterError),
                _ => Err(HexRecordError::WrongFormat),
            },
            _ => Err(HexRecordError::WrongFormat),
        }
    }
}
