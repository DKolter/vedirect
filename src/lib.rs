use hex::{HexReader, HexRecord};
use text::{TextReader, TextRecord};

mod hex;
mod text;

#[cfg(test)]
mod tests;

pub enum VedirectReader {
    Idle,
    TextReader(TextReader),
    HexReader(HexReader),
}

impl VedirectReader {
    pub fn new() -> VedirectReader {
        VedirectReader::Idle
    }

    pub fn process_byte(&mut self, byte: u8) -> Option<VedirectRecord> {
        match self {
            _ if byte == b':' && !self.is_text_checksum() => {
                println!("Switching to hex reader");
                *self = VedirectReader::HexReader(HexReader::new());
                None
            }
            Self::Idle if byte == b'\n' => {
                println!("Switching to text reader");
                *self = Self::TextReader(TextReader::new());
                None
            }
            Self::TextReader(reader) => reader.process_byte(byte).map(|record| {
                *self = VedirectReader::Idle;
                VedirectRecord::TextRecord(record)
            }),
            Self::HexReader(reader) => reader.process_byte(byte).map(|record| {
                *self = VedirectReader::Idle;
                VedirectRecord::HexRecord(record)
            }),
            _ => None,
        }
    }

    fn is_text_checksum(&self) -> bool {
        match self {
            Self::TextReader(text_reader) => text_reader.is_checksum_mode(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum VedirectRecord {
    TextRecord(TextRecord),
    HexRecord(HexRecord),
}
