pub use record::TextRecord;

mod record;

pub struct TextReader {
    checksum: u8,
    state: TextReaderState,
    record: TextRecord,
}

impl TextReader {
    pub fn new() -> TextReader {
        TextReader {
            checksum: 0,
            state: TextReaderState::RecordName(String::new()),
            record: TextRecord::new(),
        }
    }

    pub fn process_byte(&mut self, byte: u8) -> Option<TextRecord> {
        let current_state = std::mem::replace(&mut self.state, TextReaderState::Checksum);

        if !self.is_checksum_mode() {
            self.checksum = self.checksum.wrapping_add(byte);
        }

        match (current_state, byte) {
            (TextReaderState::RecordName(name), b'\t') => {
                println!("Recorded text name: {name}");
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
            (TextReaderState::RecordValue(name, value), b'\n') => {
                println!("Recorded text name and value: {name} = {value}");
                self.state = TextReaderState::RecordName(String::new());
                self.record.add_name_value(name, value);
                None
            }
            (TextReaderState::RecordValue(name, mut value), byte) => {
                value.push((byte as char).to_ascii_uppercase());
                self.state = TextReaderState::RecordValue(name, value);
                None
            }
            (TextReaderState::Checksum, byte) => {
                println!("Received checksum: {}", byte);
                println!("Calculated checksum: {}", self.checksum);
                Some(self.record.take())
            }
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
