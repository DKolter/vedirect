pub enum HexCommand {
    Ping,
    Restart,
    MpptCommand(MpptCommand),
    BmvCommand(BmvCommand),
    PhoenixCommand(PhoenixCommand),
}

pub enum MpptCommand {
    ChargerMode(bool),
    LoadMode(bool),
}

pub enum BmvCommand {}
pub enum PhoenixCommand {}

impl HexCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            HexCommand::Ping => ":154\n".as_bytes().to_vec(),
            HexCommand::Restart => ":64F\n".as_bytes().to_vec(),
            HexCommand::MpptCommand(mppt) => mppt.to_bytes(),
            HexCommand::BmvCommand(bmv) => bmv.to_bytes(),
            HexCommand::PhoenixCommand(phoenix) => phoenix.to_bytes(),
        }
    }
}

impl MpptCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MpptCommand::ChargerMode(enable) => set_command(0x0200, &[*enable as u8]),
            MpptCommand::LoadMode(enable) => set_command(0xEDA8, &[*enable as u8]),
        }
    }
}

impl BmvCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl PhoenixCommand {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

fn set_command(register: u16, value: &[u8]) -> Vec<u8> {
    let mut command = vec![b':', b'8'];
    let mut checksum = 8u8;
    command.push(u8_to_hex_char(((register & 0x00F0) >> 4) as u8));
    command.push(u8_to_hex_char((register & 0x000F) as u8));
    checksum = checksum.wrapping_add((register & 0x00FF) as u8);
    command.push(u8_to_hex_char(((register & 0xF000) >> 12) as u8));
    command.push(u8_to_hex_char(((register & 0x0F00) >> 8) as u8));
    checksum = checksum.wrapping_add(((register & 0xFF00) >> 8) as u8);
    command.extend([b'0', b'0']);
    for value in value {
        let higher = u8_to_hex_char(*value >> 4);
        let lower = u8_to_hex_char(*value & 0x0F);
        command.push(higher);
        command.push(lower);
        checksum = checksum.wrapping_add(*value);
    }

    checksum = 0x55u8.wrapping_sub(checksum);
    command.push(u8_to_hex_char(checksum >> 4));
    command.push(u8_to_hex_char(checksum & 0x0F));
    command.push(b'\n');

    command
}

fn u8_to_hex_char(hex: u8) -> u8 {
    match hex {
        0..=9 => hex + b'0',
        10..=15 => hex - 10 + b'A',
        _ => 0,
    }
}
