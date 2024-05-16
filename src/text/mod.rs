pub use record::TextRecord;

mod record;

pub struct TextReader {
    state: TextReaderState,
    record: TextRecord,
}

impl TextReader {
    pub fn new() -> TextReader {
        TextReader {
            state: TextReaderState::RecordName(String::new()),
            record: TextRecord::new(),
        }
    }

    pub fn process_byte(&mut self, byte: u8) -> Option<TextRecord> {
        let current_state = std::mem::replace(&mut self.state, TextReaderState::Checksum);

        match (current_state, byte) {
            (TextReaderState::RecordName(name), b'\t') => {
                self.state = match name.as_str() {
                    "CHECKSUM" => TextReaderState::Checksum,
                    _ => TextReaderState::RecordValue(name, String::new()),
                };
                None
            }
            (TextReaderState::RecordName(mut name), byte) => {
                name.push((byte as char).to_ascii_uppercase());
                self.state = TextReaderState::RecordName(name);
                None
            }
            (TextReaderState::RecordValue(name, value), b'\r') => {
                self.state = TextReaderState::RecordValue(name, value);
                None
            }
            (TextReaderState::RecordValue(name, value), b'\n') => {
                self.state = TextReaderState::RecordName(String::new());
                self.record.add_name_value(name, value);
                None
            }
            (TextReaderState::RecordValue(name, mut value), byte) => {
                value.push((byte as char).to_ascii_uppercase());
                self.state = TextReaderState::RecordValue(name, value);
                None
            }
            (TextReaderState::Checksum, _) => Some(self.record.take()),
        }
    }

    pub fn is_checksum_mode(&self) -> bool {
        self.state == TextReaderState::Checksum
    }
}

#[derive(PartialEq, Eq)]
enum TextReaderState {
    RecordName(String),
    RecordValue(String, String),
    Checksum,
}
