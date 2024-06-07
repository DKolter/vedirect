use super::HexRecordError;

enum RegisterFlags {
    Ok,
    UnknownId,
    NotSupported,
    ParameterError,
}

impl RegisterFlags {
    pub fn from_byte(byte: u8) -> Result<Self, HexRecordError> {
        match byte {
            0 => Ok(Self::Ok),
            1 => Ok(Self::UnknownId),
            2 => Ok(Self::NotSupported),
            3 => Ok(Self::ParameterError),
            _ => Err(HexRecordError::WrongFormat),
        }
    }
}

pub enum HexRegister {
    ProductId,
    GroupId,
    SerialNumber,
    ModelName,
    Capabilities,
    DeviceMode,
    DeviceState,
    RemoteControlUsed,
    DeviceOffReason1,
    DeviceOffReason2,
    BatterySafeMode,
    AdaptiveMode,
    AutomaticEqualisationMode,
    BatteryBulkTimeLimit,
    BatteryAbsorptionTimeLimit,
    BatteryAbsorptionVoltage,
    BatteryFloatVoltage,
    BatteryEqualisationVoltage,
    BatteryTempCompensation,
    BatteryType,
    BatteryMaximumCurrent,
    BatteryVoltage,
    BatteryTemperature,
    BatteryVoltageSetting,
    BmsPresent,
    TailCurrent,
    LowTemperatureChargeCurrent,
}
