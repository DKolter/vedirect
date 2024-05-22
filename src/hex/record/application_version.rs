use super::HexRecordError;

#[derive(Debug)]
pub struct ApplicationVersion {
    pub firmware_type: FirmwareType,
    pub major: u8,
    pub minor: u8,
}

impl ApplicationVersion {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HexRecordError> {
        if bytes.len() != 2 {
            return Err(HexRecordError::WrongFormat);
        }

        let firmware_type = FirmwareType::from_byte(bytes[0]);
        let major = bytes[0] & 0b0000_1111;
        let minor = bytes[1];

        Ok(Self {
            firmware_type,
            major,
            minor,
        })
    }
}

#[derive(Debug)]
pub enum FirmwareType {
    Bootloader,
    Application,
    Tester,
    Release(u8),
}

impl FirmwareType {
    pub fn from_byte(byte: u8) -> Self {
        match byte & 0b1100_0000 {
            0b0000_0000 => Self::Bootloader,
            0b0100_0000 => Self::Application,
            0b1000_0000 => Self::Tester,
            0b1100_0000 => Self::Release(byte & 0b1111_0000),
            _ => unreachable!(),
        }
    }
}
