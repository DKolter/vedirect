use hex::{HexReader, HexRecord, HexRecordError};
use text::{TextReader, TextRecord, TextRecordError};

mod hex;
mod text;

#[cfg(test)]
mod tests;

pub struct VedirectReader {
    checksum: u8,
    state: VedirectReaderState,
}

impl VedirectReader {
    pub fn new() -> VedirectReader {
        Self {
            checksum: 0,
            state: VedirectReaderState::Idle,
        }
    }

    pub fn process_byte(&mut self, byte: u8) -> Result<Option<VedirectRecord>, VedirectError> {
        let current_state = std::mem::take(&mut self.state);
        match current_state {
            _ if byte == b':' && !self.is_text_checksum() => {
                self.state = VedirectReaderState::HexReader(HexReader::new());
                Ok(None)
            }
            VedirectReaderState::Idle => {
                self.checksum = self.checksum.wrapping_add(byte);
                if byte == b'\n' {
                    self.state = VedirectReaderState::TextReader(TextReader::new());
                }
                Ok(None)
            }
            VedirectReaderState::TextReader(mut reader) => {
                self.checksum = self.checksum.wrapping_add(byte);
                let record = reader.process_byte(byte);

                match record {
                    Ok(Some(record)) => {
                        self.state = VedirectReaderState::Idle;
                        match self.checksum {
                            0 => Ok(Some(VedirectRecord::TextRecord(record))),
                            _ => {
                                self.checksum = 0;
                                Err(VedirectError::TextRecordError(
                                    TextRecordError::ChecksumError,
                                ))
                            }
                        }
                    }
                    Ok(None) => {
                        self.state = VedirectReaderState::TextReader(reader);
                        Ok(None)
                    }
                    Err(err) => {
                        self.state = VedirectReaderState::Idle;
                        self.checksum = 0;
                        Err(VedirectError::TextRecordError(err))
                    }
                }
            }
            VedirectReaderState::HexReader(mut reader) => {
                let record = reader.process_byte(byte);
                match record {
                    Ok(Some(record)) => {
                        self.state = VedirectReaderState::Idle;
                        self.checksum = 0;
                        Ok(Some(VedirectRecord::HexRecord(record)))
                    }
                    Ok(None) => {
                        self.state = VedirectReaderState::HexReader(reader);
                        Ok(None)
                    }
                    Err(err) => {
                        self.state = VedirectReaderState::Idle;
                        self.checksum = 0;
                        Err(VedirectError::HexRecordError(err))
                    }
                }
            }
        }
    }

    fn is_text_checksum(&self) -> bool {
        match &self.state {
            VedirectReaderState::TextReader(text_reader) => text_reader.is_checksum_mode(),
            _ => false,
        }
    }
}

#[derive(Default)]
pub enum VedirectReaderState {
    #[default]
    Idle,
    TextReader(TextReader),
    HexReader(HexReader),
}

#[derive(Debug)]
pub enum VedirectRecord {
    TextRecord(TextRecord),
    HexRecord(HexRecord),
}

#[derive(Debug)]
pub enum VedirectError {
    TextRecordError(TextRecordError),
    HexRecordError(HexRecordError),
}
