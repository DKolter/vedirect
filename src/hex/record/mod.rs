use application_version::ApplicationVersion;

mod application_version;

#[derive(Debug)]
pub enum HexRecordError {
    UnknownResponse,
    CheckSumError,
}

#[derive(Debug)]
pub enum HexRecord {
    Ping(ApplicationVersion),
}

impl HexRecord {
    pub fn from_bytes(buffer: &[u8]) -> Result<Self, HexRecordError> {
        let buffer = Self::parse_ascii_hex(buffer);

        if !Self::checksum_correct(&buffer) {
            return Err(HexRecordError::CheckSumError);
        }

        match buffer.as_slice() {
            [5, version @ .., _, _] => Ok(Self::Ping(ApplicationVersion::from_bytes(version)?)),
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
