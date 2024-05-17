use super::TextRecordError;

#[derive(Debug)]
pub enum MonitorMode {
    SolarCharger,
    WindTurbine,
    ShaftGenerator,
    Alternator,
    FuelCell,
    WaterGenerator,
    DcDcCharger,
    AcCharger,
    GenericSource,
    BatteryMonitor,
    GenericLoad,
    ElectricDrive,
    Fridge,
    WaterPump,
    BilgePump,
    DcSystem,
    Inverter,
    WaterHeater,
}

impl MonitorMode {
    pub fn parse(value: &str) -> Result<MonitorMode, TextRecordError> {
        Ok(match value {
            "-9" => MonitorMode::SolarCharger,
            "-8" => MonitorMode::WindTurbine,
            "-7" => MonitorMode::ShaftGenerator,
            "-6" => MonitorMode::Alternator,
            "-5" => MonitorMode::FuelCell,
            "-4" => MonitorMode::WaterGenerator,
            "-3" => MonitorMode::DcDcCharger,
            "-2" => MonitorMode::AcCharger,
            "-1" => MonitorMode::GenericSource,
            "0" => MonitorMode::BatteryMonitor,
            "1" => MonitorMode::GenericLoad,
            "2" => MonitorMode::ElectricDrive,
            "3" => MonitorMode::Fridge,
            "4" => MonitorMode::WaterPump,
            "5" => MonitorMode::BilgePump,
            "6" => MonitorMode::DcSystem,
            "7" => MonitorMode::Inverter,
            "8" => MonitorMode::WaterHeater,
            _ => return Err(TextRecordError::ParseError),
        })
    }
}
