pub enum DeviceMode {
    InverterOn,
    DeviceOn,
    DeviceOff,
    EcoMode,
    Hibernate,
}

impl DeviceMode {
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            DeviceMode::InverterOn => &[2],
            DeviceMode::DeviceOn => &[3],
            DeviceMode::DeviceOff => &[4],
            DeviceMode::EcoMode => &[5],
            DeviceMode::Hibernate => &[0xFD],
        }
    }
}
