#[derive(Debug)]
pub enum HexRecordError {}

#[derive(Debug)]
pub struct HexRecord {
    buffer: Vec<u8>,
}

impl HexRecord {
    pub fn from_bytes(buffer: Vec<u8>) -> Self {
        Self { buffer }
    }
}
