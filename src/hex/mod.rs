pub use record::{HexRecord, HexRecordError};

mod record;

pub struct HexReader {
    buffer: Vec<u8>,
}

impl HexReader {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn process_byte(&mut self, byte: u8) -> Result<Option<HexRecord>, HexRecordError> {
        match byte {
            b'\n' => Ok(Some(HexRecord::from_bytes(std::mem::take(
                &mut self.buffer,
            )))),
            b'\r' => Ok(None),
            byte => {
                self.buffer.push(byte);
                Ok(None)
            }
        }
    }
}
