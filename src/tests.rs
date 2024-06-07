use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::SerialPortBuilderExt;

use crate::{HexCommand, LoadMode, MpptCommand, VedirectReader};

#[tokio::test]
async fn test_ttys0() {
    let mut serial = tokio_serial::new("/dev/ttyS0", 19200)
        .open_native_async()
        .expect("Failed to open serial port");

    let command = HexCommand::MpptCommand(MpptCommand::LoadMode(LoadMode::Auto));
    serial
        .write(&command.to_bytes())
        .await
        .expect("Failed to write command");

    let mut reader = VedirectReader::new();
    loop {
        let byte = serial.read_u8().await.expect("Failed to read byte");
        match reader.process_byte(byte) {
            Ok(Some(record)) => println!("{:?}", record),
            Err(err) => eprintln!("{:?}", err),
            _ => {}
        }
    }
}
