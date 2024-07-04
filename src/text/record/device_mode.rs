use super::TextRecordError;

#[derive(Debug, PartialEq, Eq)]
pub enum DeviceMode {
    VeRegModeCharger,
    VeRegModeInverter,
    VeRegModeOff,
    VeRegModeEco,
    VeRegModeHibernate,
}

impl DeviceMode {
    pub fn parse(value: &str) -> Result<DeviceMode, TextRecordError> {
        match value {
            "VE_REG_MODE_CHARGER" => Ok(DeviceMode::VeRegModeCharger),
            "VE_REG_MODE_INVERTER" => Ok(DeviceMode::VeRegModeInverter),
            "VE_REG_MODE_OFF" => Ok(DeviceMode::VeRegModeOff),
            "VE_REG_MODE_ECO" => Ok(DeviceMode::VeRegModeEco),
            "VE_REG_MODE_HIBERNATE" => Ok(DeviceMode::VeRegModeHibernate),
            _ => Err(TextRecordError::ParseError),
        }
    }
}
