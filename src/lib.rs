use hex::{HexReader, HexRecord};
use text::{TextReader, TextRecord};

mod hex;
mod text;

#[cfg(test)]
mod tests;

pub enum VedirectReader {
    Idle(u8),
    TextReader(TextReader),
    HexReader(HexReader),
}

impl VedirectReader {
    pub fn new() -> VedirectReader {
        VedirectReader::Idle(0)
    }

    pub fn process_byte(&mut self, byte: u8) -> Option<VedirectRecord> {
        match self {
            _ if byte == b':' && !self.is_text_checksum() => {
                println!("Switching to hex reader");
                *self = VedirectReader::HexReader(HexReader::new());
                None
            }
            Self::Idle(ref mut checksum) => {
                *checksum = checksum.wrapping_add(byte);
                if byte == b'\n' {
                    println!("Switching to text reader");
                    *self = Self::TextReader(TextReader::new(*checksum));
                }
                None
            }
            Self::TextReader(reader) => reader.process_byte(byte).map(|record| {
                *self = VedirectReader::Idle(0);
                VedirectRecord::TextRecord(record)
            }),
            Self::HexReader(reader) => reader.process_byte(byte).map(|record| {
                *self = VedirectReader::Idle(0);
                VedirectRecord::HexRecord(record)
            }),
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
