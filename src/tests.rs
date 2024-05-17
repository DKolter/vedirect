use tokio::io::AsyncReadExt;
use tokio_serial::SerialPortBuilderExt;

use crate::VedirectReader;

#[tokio::test]
async fn test_ttys0() {
    let mut serial = tokio_serial::new("/dev/ttyS0", 19200)
        .open_native_async()
        .expect("Failed to open serial port");

    let mut reader = VedirectReader::new();
    loop {
        let byte = serial.read_u8().await.expect("Failed to read byte");
        if let Some(record) = reader.process_byte(byte).unwrap() {
            println!("{:?}", record);
        }
    }
}
