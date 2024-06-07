use application_version::ApplicationVersion;
use error_type::ErrorType;
use set_response::SetResponse;

mod application_version;
mod error_type;
mod set_response;

#[derive(Debug)]
pub enum HexRecordError {
    CheckSumError,
    WrongFormat,
    UnknownResponse,
}

#[derive(Debug)]
pub enum HexRecord {
    Done,
    UnknownCommand,
    Error(ErrorType),
    Ping(ApplicationVersion),
    SetResponse(SetResponse),
}

impl HexRecord {
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, HexRecordError> {
        let buffer = Self::parse_ascii_hex(buffer);

        if !Self::checksum_correct(&buffer) {
            return Err(HexRecordError::CheckSumError);
        }

        match buffer.as_slice() {
            [1, ..] => Ok(Self::Done),
            [3, ..] => Ok(Self::UnknownCommand),
            [4, error_type @ .., _] => Ok(Self::Error(ErrorType::from_bytes(error_type)?)),
            [5, version @ .., _] => Ok(Self::Ping(ApplicationVersion::from_bytes(version)?)),
            [8, data @ .., _] => Ok(Self::SetResponse(SetResponse::from_bytes(data)?)),
            _ => Err(HexRecordError::UnknownResponse),
        }
    }

    fn parse_ascii_hex(buffer: &[u8]) -> Vec<u8> {
        match buffer {
            [] => Vec::new(),
            [command, rest @ ..] => {
                let mut buffer = vec![Self::hex_char_to_u8(*command)];
                buffer.extend(rest.chunks_exact(2).map(|chunk| {
                    let higher = Self::hex_char_to_u8(chunk[0]);
                    let lower = Self::hex_char_to_u8(chunk[1]);
                    (higher << 4) + lower
                }));
                buffer
            }
        }
    }

    fn checksum_correct(buffer: &[u8]) -> bool {
        buffer.iter().fold(0u8, |acc, x| acc.wrapping_add(*x)) == 0x55
    }

    fn hex_char_to_u8(hex: u8) -> u8 {
        match hex {
            b'0'..=b'9' => hex - b'0',
            b'A'..=b'F' => hex - b'A' + 10,
            _ => 0,
        }
    }
}
