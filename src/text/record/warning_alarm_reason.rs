use super::TextRecordError;

#[derive(Debug)]
pub struct WarningAlarmReason {
    pub low_voltage: bool,
    pub high_voltage: bool,
    pub low_soc: bool,
    pub low_starter_voltage: bool,
    pub high_starter_voltage: bool,
    pub low_temperature: bool,
    pub high_temperature: bool,
    pub mid_voltage: bool,
    pub overload: bool,
    pub dc_ripple: bool,
    pub low_vac_out: bool,
    pub high_vac_out: bool,
    pub short_circuit: bool,
    pub bms_lockout: bool,
}

impl WarningAlarmReason {
    pub fn parse(value: &str) -> Result<WarningAlarmReason, TextRecordError> {
        let value: u32 = value.parse().map_err(|_| TextRecordError::ParseError)?;

        Ok(WarningAlarmReason {
            low_voltage: value & 1 != 0,
            high_voltage: value & 2 != 0,
            low_soc: value & 4 != 0,
            low_starter_voltage: value & 8 != 0,
            high_starter_voltage: value & 16 != 0,
            low_temperature: value & 32 != 0,
            high_temperature: value & 64 != 0,
            mid_voltage: value & 128 != 0,
            overload: value & 256 != 0,
            dc_ripple: value & 512 != 0,
            low_vac_out: value & 1024 != 0,
            high_vac_out: value & 2048 != 0,
            short_circuit: value & 4096 != 0,
            bms_lockout: value & 8192 != 0,
        })
    }
}
