use super::TextRecordError;

#[derive(Debug)]
pub enum ErrorCode {
    NoError,
    BatteryVoltageTooHigh,
    ChargerTemperatureTooHigh,
    ChargerOverCurrent,
    ChargerCurrentReversed,
    BulkTimeLimitExceeded,
    CurrentSensorIssue,
    TerminalsOverheated,
    ConverterIssue,
    InputVoltageTooHigh,
    InputCurrentTooHigh,
    InputShutdownBatteryVoltage,
    InputShutdownCurrentFlow,
    LostCommunication,
    SyncChargingDeviceConfigIssue,
    BmsConnectionLost,
    NetworkMisconfigured,
    FactoryCalibrationDataLost,
    InvalidFirmware,
    UserSettingsInvalid,
}

impl ErrorCode {
    pub fn parse(value: &str) -> Result<ErrorCode, TextRecordError> {
        Ok(match value {
            "0" => ErrorCode::NoError,
            "2" => ErrorCode::BatteryVoltageTooHigh,
            "17" => ErrorCode::ChargerTemperatureTooHigh,
            "18" => ErrorCode::ChargerOverCurrent,
            "19" => ErrorCode::ChargerCurrentReversed,
            "20" => ErrorCode::BulkTimeLimitExceeded,
            "21" => ErrorCode::CurrentSensorIssue,
            "26" => ErrorCode::TerminalsOverheated,
            "28" => ErrorCode::ConverterIssue,
            "33" => ErrorCode::InputVoltageTooHigh,
            "34" => ErrorCode::InputCurrentTooHigh,
            "38" => ErrorCode::InputShutdownBatteryVoltage,
            "39" => ErrorCode::InputShutdownCurrentFlow,
            "65" => ErrorCode::LostCommunication,
            "66" => ErrorCode::SyncChargingDeviceConfigIssue,
            "67" => ErrorCode::BmsConnectionLost,
            "68" => ErrorCode::NetworkMisconfigured,
            "116" => ErrorCode::FactoryCalibrationDataLost,
            "117" => ErrorCode::InvalidFirmware,
            "119" => ErrorCode::UserSettingsInvalid,
            _ => return Err(TextRecordError::ParseError),
        })
    }
}
