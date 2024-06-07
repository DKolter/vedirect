pub enum LoadMode {
    // Load output off
    Off,
    // Automatic control / batterylife (default)
    Auto,
    // Alternative control 1 (off < 11.1V, on > 13.1V)
    Alt1,
    // Alternative control 2 (off < 11.8V, on > 14.0V)
    Alt2,
    // Load output on (use with caution, no discharge guard)
    On,
    // User defined settings 1 (off < Vlow, on > Vhigh)
    User1,
    // User defined settings 2 (off < Vlow, on > Vhigh)
    User2,
    // Automatic energy selector
    Aes,
}

impl LoadMode {
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            LoadMode::Off => &[0],
            LoadMode::Auto => &[1],
            LoadMode::Alt1 => &[2],
            LoadMode::Alt2 => &[3],
            LoadMode::On => &[4],
            LoadMode::User1 => &[5],
            LoadMode::User2 => &[6],
            LoadMode::Aes => &[7],
        }
    }
}
