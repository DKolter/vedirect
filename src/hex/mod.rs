pub use record::HexRecord;

mod record;

pub struct HexReader;

impl HexReader {
    pub fn new() -> HexReader {
        HexReader
    }

    pub fn process_byte(&mut self, _byte: u8) -> Option<HexRecord> {
        None
    }
}
