pub use charger_status::ChargerStatus;
pub use device_mode::DeviceMode;
pub use error_code::ErrorCode;
pub use monitor_mode::MonitorMode;
pub use mppt_mode::MpptMode;
pub use off_reason::OffReason;
pub use product_id::ProductId;
pub use warning_alarm_reason::WarningAlarmReason;

mod charger_status;
mod device_mode;
mod error_code;
mod monitor_mode;
mod mppt_mode;
mod off_reason;
mod product_id;
mod warning_alarm_reason;

#[derive(Debug)]
pub enum TextRecordError {
    UnknownField,
    ParseError,
    ChecksumError,
}

#[derive(Debug, Default)]
pub struct TextRecord {
    /// Main or channel 1 (battery) voltage in mV
    pub v: Option<i32>,
    /// Channel 2 (battery) voltage in mV
    pub v2: Option<i32>,
    /// Channel 3 (battery) voltage in mV
    pub v3: Option<i32>,
    /// Auxiliary (starter) voltage in mV
    pub vs: Option<i32>,
    /// Mid-point voltage of the battery bank in mV
    pub vm: Option<i32>,
    /// Mid-point deviation of the battery bank in percentage
    pub dm: Option<i32>,
    /// Panel voltage in mV
    pub vpv: Option<i32>,
    /// Panel power in W
    pub ppv: Option<i32>,
    /// Main or channel 1 battery current in mA
    pub i: Option<i32>,
    /// Channel 2 battery current in mA
    pub i2: Option<i32>,
    /// Channel 3 battery current in mA
    pub i3: Option<i32>,
    /// Load current in mA
    pub il: Option<i32>,
    /// Load output state (ON = true, OFF = false)
    pub load: Option<bool>,
    /// Battery temperature in degrees Celsius
    pub t: Option<i32>,
    /// Instantaneous power in W
    pub p: Option<i32>,
    /// Consumed amphours in mAh
    pub ce: Option<i32>,
    /// State of charge
    pub soc: Option<i32>,
    /// Time to go in minutes
    pub ttg: Option<i32>,
    /// Alarm condition active (ON = true, OFF = false)
    pub alarm: Option<bool>,
    /// Relay state
    pub relay: Option<bool>,
    /// Alarm reason
    pub ar: Option<WarningAlarmReason>,
    /// Off reason
    pub or: Option<OffReason>,
    /// Depth of the deepest discharge in mAh
    pub h1: Option<i32>,
    /// Depth of the last discharge in mAh
    pub h2: Option<i32>,
    /// Depth of the average discharge in mAh
    pub h3: Option<i32>,
    /// Number of charge cycles
    pub h4: Option<i32>,
    /// Number of full discharges
    pub h5: Option<i32>,
    /// Cumulative amp hours drawn in mAh
    pub h6: Option<i32>,
    /// Minimum main (battery) voltage in mV
    pub h7: Option<i32>,
    /// Maximum main (battery) voltage in mV
    pub h8: Option<i32>,
    /// Number of seconds since last full charge
    pub h9: Option<i32>,
    /// Number of automatic synchronizations
    pub h10: Option<i32>,
    /// Number of low main voltage alarms
    pub h11: Option<i32>,
    /// Number of high main voltage alarms
    pub h12: Option<i32>,
    /// Number of low auxiliary voltage alarms
    pub h13: Option<i32>,
    /// Number of high auxiliary voltage alarms
    pub h14: Option<i32>,
    /// Minimum auxiliary (battery) voltage in mV
    pub h15: Option<i32>,
    /// Maximum auxiliary (battery) voltage in mV
    pub h16: Option<i32>,
    /// Amount of discharged energy (BMV) / Amount of charged energy (DC monitor) in 0.01 kWh
    pub h17: Option<i32>,
    /// Amount of charged energy (BMV) / Amount of consumed energy (DC monitor) in 0.01 kWh
    pub h18: Option<i32>,
    /// Yield total (user resettable counter) in 0.01 kWh
    pub h19: Option<i32>,
    /// Yield today in 0.01 kWh
    pub h20: Option<i32>,
    /// Maximum power today in W
    pub h21: Option<i32>,
    /// Yield yesterday in 0.01 kWh
    pub h22: Option<i32>,
    /// Maximum power yesterday in W
    pub h23: Option<i32>,
    /// Error code
    pub err: Option<ErrorCode>,
    ///  State of operation
    pub cs: Option<ChargerStatus>,
    /// Model description
    pub bmv: Option<String>,
    /// Firmware version (16 bit)
    pub fw: Option<String>,
    /// Firmware version (24 bit)
    pub fwe: Option<String>,
    /// Product ID
    pub pid: Option<ProductId>,
    /// Serial number
    pub ser: Option<String>,
    /// Day sequence number
    pub hsds: Option<i32>,
    /// Device mode
    pub mode: Option<DeviceMode>,
    /// AC output voltage in 0.01 kWh
    pub ac_out_v: Option<i32>,
    /// AC output current in 0.1 A
    pub ac_out_i: Option<i32>,
    /// AC output apparent power in VA
    pub ac_out_s: Option<i32>,
    /// Warning reason
    pub warn: Option<WarningAlarmReason>,
    /// Tracker operation mode
    pub mppt: Option<MpptMode>,
    /// DC monitor mode
    pub mon: Option<MonitorMode>,
    // DC input voltage in 0.01V
    pub dc_in_v: Option<i32>,
    // DC input current in 0.1A
    pub dc_in_i: Option<i32>,
    // DC input power in W
    pub dc_in_p: Option<i32>,
}

impl TextRecord {
    pub fn add_name_value(&mut self, name: String, value: String) -> Result<(), TextRecordError> {
        match name.as_str() {
            "V" => self.v = Some(parse_number(&value)?),
            "V2" => self.v2 = Some(parse_number(&value)?),
            "V3" => self.v3 = Some(parse_number(&value)?),
            "VS" => self.vs = Some(parse_number(&value)?),
            "VM" => self.vm = Some(parse_number(&value)?),
            "DM" => self.dm = Some(parse_number(&value)?),
            "VPV" => self.vpv = Some(parse_number(&value)?),
            "PPV" => self.ppv = Some(parse_number(&value)?),
            "I" => self.i = Some(parse_number(&value)?),
            "I2" => self.i2 = Some(parse_number(&value)?),
            "I3" => self.i3 = Some(parse_number(&value)?),
            "IL" => self.il = Some(parse_number(&value)?),
            "LOAD" => self.load = Some(parse_on_off(&value)?),
            "T" => self.t = parse_number_maybe_triple_dash(&value)?,
            "P" => self.p = Some(parse_number(&value)?),
            "CE" => self.ce = parse_number_maybe_triple_dash(&value)?,
            "SOC" => self.soc = parse_number_maybe_triple_dash(&value)?,
            "TTG" => self.ttg = parse_number_maybe_triple_dash(&value)?,
            "ALARM" => self.alarm = Some(parse_on_off(&value)?),
            "RELAY" => self.relay = Some(parse_on_off(&value)?),
            "AR" => self.ar = Some(WarningAlarmReason::parse(&value)?),
            "OR" => self.or = Some(OffReason::parse(&value)?),
            "H1" => self.h1 = parse_number_maybe_triple_dash(&value)?,
            "H2" => self.h2 = parse_number_maybe_triple_dash(&value)?,
            "H3" => self.h3 = parse_number_maybe_triple_dash(&value)?,
            "H4" => self.h4 = parse_number_maybe_triple_dash(&value)?,
            "H5" => self.h5 = parse_number_maybe_triple_dash(&value)?,
            "H6" => self.h6 = parse_number_maybe_triple_dash(&value)?,
            "H7" => self.h7 = Some(parse_number(&value)?),
            "H8" => self.h8 = Some(parse_number(&value)?),
            "H9" => self.h9 = parse_number_maybe_triple_dash(&value)?,
            "H10" => self.h10 = parse_number_maybe_triple_dash(&value)?,
            "H11" => self.h11 = Some(parse_number(&value)?),
            "H12" => self.h12 = Some(parse_number(&value)?),
            "H13" => self.h13 = Some(parse_number(&value)?),
            "H14" => self.h14 = Some(parse_number(&value)?),
            "H15" => self.h15 = Some(parse_number(&value)?),
            "H16" => self.h16 = Some(parse_number(&value)?),
            "H17" => self.h17 = Some(parse_number(&value)?),
            "H18" => self.h18 = Some(parse_number(&value)?),
            "H19" => self.h19 = Some(parse_number(&value)?),
            "H20" => self.h20 = Some(parse_number(&value)?),
            "H21" => self.h21 = Some(parse_number(&value)?),
            "H22" => self.h22 = Some(parse_number(&value)?),
            "H23" => self.h23 = Some(parse_number(&value)?),
            "ERR" => self.err = Some(ErrorCode::parse(&value)?),
            "CS" => self.cs = Some(ChargerStatus::parse(&value)?),
            "BMV" => self.bmv = Some(value),
            "FW" => self.fw = Some(value),
            "FWE" => self.fwe = Some(value),
            "PID" => self.pid = Some(ProductId::parse(&value)?),
            "SER#" => self.ser = Some(value),
            "HSDS" => self.hsds = Some(parse_number(&value)?),
            "MODE" => self.mode = Some(DeviceMode::parse(&value)?),
            "AC_OUT_V" => self.ac_out_v = Some(parse_number(&value)?),
            "AC_OUT_I" => self.ac_out_i = Some(parse_number(&value)?),
            "AC_OUT_S" => self.ac_out_s = Some(parse_number(&value)?),
            "WARN" => self.warn = Some(WarningAlarmReason::parse(&value)?),
            "MPPT" => self.mppt = Some(MpptMode::parse(&value)?),
            "MON" => self.mon = Some(MonitorMode::parse(&value)?),
            "DC_IN_V" => self.dc_in_v = Some(parse_number(&value)?),
            "DC_IN_I" => self.dc_in_i = Some(parse_number(&value)?),
            "DC_IN_P" => self.dc_in_p = Some(parse_number(&value)?),
            _ => return Err(TextRecordError::UnknownField),
        }

        Ok(())
    }

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}

fn parse_number(value: &str) -> Result<i32, TextRecordError> {
    Ok(value.parse().map_err(|_| TextRecordError::ParseError)?)
}

fn parse_on_off(value: &str) -> Result<bool, TextRecordError> {
    match value {
        "ON" => Ok(true),
        "OFF" => Ok(false),
        _ => Err(TextRecordError::ParseError),
    }
}

fn parse_number_maybe_triple_dash(value: &str) -> Result<Option<i32>, TextRecordError> {
    Ok(match value {
        "---" => None,
        _ => Some(value.parse().map_err(|_| TextRecordError::ParseError)?),
    })
}
