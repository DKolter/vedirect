use super::TextRecordError;

#[derive(Debug)]
pub enum OffReason {
    NoInputPower,
    SwitchedOffPowerSwitch,
    SwitchedOffDeviceModeRegister,
    RemoteInput,
    ProtectionActive,
    Paygo,
    Bms,
    EngineShutdownDetection,
    AnalyingInputVoltage,
}

impl OffReason {
    pub fn parse(value: &str) -> Result<OffReason, TextRecordError> {
        Ok(match value {
            "0X00000001" => OffReason::NoInputPower,
            "0X00000002" => OffReason::SwitchedOffPowerSwitch,
            "0X00000004" => OffReason::SwitchedOffDeviceModeRegister,
            "0X00000008" => OffReason::RemoteInput,
            "0X00000010" => OffReason::ProtectionActive,
            "0X00000020" => OffReason::Paygo,
            "0X00000040" => OffReason::Bms,
            "0X00000080" => OffReason::EngineShutdownDetection,
            "0X00000100" => OffReason::AnalyingInputVoltage,
            _ => return Err(TextRecordError::ParseError),
        })
    }
}
