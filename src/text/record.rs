#[derive(Debug)]
pub struct TextRecord {
    fields: Vec<(String, String)>,
}

impl TextRecord {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn add_name_value(&mut self, name: String, value: String) {
        self.fields.push((name, value));
    }

    pub fn take(&mut self) -> Self {
        Self {
            fields: std::mem::take(&mut self.fields),
        }
    }
}
