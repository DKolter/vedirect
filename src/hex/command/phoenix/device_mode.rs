pub enum PhoenixDeviceMode {
    InverterOn,
    DeviceOn,
    DeviceOff,
    EcoMode,
    Hibernate,
}

impl PhoenixDeviceMode {
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            PhoenixDeviceMode::InverterOn => &[2],
            PhoenixDeviceMode::DeviceOn => &[3],
            PhoenixDeviceMode::DeviceOff => &[4],
            PhoenixDeviceMode::EcoMode => &[5],
            PhoenixDeviceMode::Hibernate => &[0xFD],
        }
    }
}
