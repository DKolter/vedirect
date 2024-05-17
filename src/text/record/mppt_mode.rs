use super::TextRecordError;

#[derive(Debug)]
pub enum MpptMode {
    Off,
    VoltageOrCurrentLimited,
    MppTrackerActive,
}

impl MpptMode {
    pub fn parse(value: &str) -> Result<MpptMode, TextRecordError> {
        Ok(match value {
            "0" => MpptMode::Off,
            "1" => MpptMode::VoltageOrCurrentLimited,
            "2" => MpptMode::MppTrackerActive,
            _ => return Err(TextRecordError::ParseError),
        })
    }
}
