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
        let buffer = buffer
            .iter()
            .copied()
            .map(Self::hex_char_to_u8)
            .collect::<Vec<_>>();

        if Self::calc_checksum(&buffer) != 0x55 {
            return Err(HexRecordError::CheckSumError);
        }

        match buffer.as_slice() {
            [b'5', version @ .., _, _] => Ok(Self::Ping(ApplicationVersion::from_bytes(version)?)),
            _ => Err(HexRecordError::UnknownResponse),
        }
    }

    fn calc_checksum(buffer: &[u8]) -> u8 {
        match buffer {
            [] => 0,
            [command, rest @ ..] => {
                command.wrapping_add(rest.iter().enumerate().fold(0, |acc, (i, byte)| {
                    match i % 2 {
                        0 => acc.wrapping_add(byte << 4),
                        _ => acc.wrapping_add(*byte),
                    }
                }))
            }
        }
    }

    fn hex_char_to_u8(hex: u8) -> u8 {
        match hex {
            b'0'..=b'9' => hex - b'0',
            b'A'..=b'F' => hex - b'A' + 10,
            _ => 0,
        }
    }
}
