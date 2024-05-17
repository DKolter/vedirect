use super::TextRecordError;

#[derive(Debug)]
pub enum ChargerStatus {
    Off,
    LowPower,
    Fault,
    Bulk,
    Absorption,
    Float,
    Storage,
    Equalize,
    Inverting,
    PowerSupply,
    StartingUp,
    RepeatedAbsorption,
    AutoEqualize,
    BatterySafe,
    ExternalControl,
}

impl ChargerStatus {
    pub fn parse(value: &str) -> Result<ChargerStatus, TextRecordError> {
        Ok(match value {
            "0" => ChargerStatus::Off,
            "1" => ChargerStatus::LowPower,
            "2" => ChargerStatus::Fault,
            "3" => ChargerStatus::Bulk,
            "4" => ChargerStatus::Absorption,
            "5" => ChargerStatus::Float,
            "6" => ChargerStatus::Storage,
            "7" => ChargerStatus::Equalize,
            "9" => ChargerStatus::Inverting,
            "11" => ChargerStatus::PowerSupply,
            "245" => ChargerStatus::StartingUp,
            "246" => ChargerStatus::RepeatedAbsorption,
            "247" => ChargerStatus::AutoEqualize,
            "248" => ChargerStatus::BatterySafe,
            "252" => ChargerStatus::ExternalControl,
            _ => return Err(TextRecordError::ParseError),
        })
    }
}
