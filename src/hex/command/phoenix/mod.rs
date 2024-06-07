use super::set_command;
pub use device_mode::DeviceMode;

mod device_mode;

pub enum PhoenixCommand {
    DeviceMode(DeviceMode),
}

impl PhoenixCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            PhoenixCommand::DeviceMode(mode) => set_command(0x0200, mode.to_bytes()),
        }
    }
}
