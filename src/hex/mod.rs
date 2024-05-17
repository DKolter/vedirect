pub use record::{HexRecord, HexRecordError};

mod record;

pub struct HexReader;

impl HexReader {
    pub fn new() -> HexReader {
        HexReader
    }

    pub fn process_byte(&mut self, _byte: u8) -> Result<Option<HexRecord>, HexRecordError> {
        Ok(None)
    }
}
