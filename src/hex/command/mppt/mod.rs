use super::set_command;
pub use load_mode::LoadMode;

mod load_mode;

pub enum MpptCommand {
    ChargerMode(bool),
    LoadMode(LoadMode),
}

impl MpptCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MpptCommand::ChargerMode(enable) => set_command(0x0200, &[*enable as u8]),
            MpptCommand::LoadMode(mode) => set_command(0xEDAB, mode.to_bytes()),
        }
    }
}
