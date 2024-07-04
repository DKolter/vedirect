use crate::{
    text::record::{OffReason, ProductId},
    TextReader,
};

#[test]
fn test_valid_smartsolar_mppt75_15_1() {
    let mut reader = TextReader::new();
    for byte in include_bytes!("test_valid_smartsolar_mppt75_15_1.txt") {
        match reader.process_byte(*byte) {
            Ok(Some(record)) => {
                assert_eq!(record.pid, Some(ProductId::SmartSolarMPPT75_15));
                assert_eq!(record.fw, Some("147".to_string()));
                assert_eq!(record.ser, Some("HQ2134AD23G".to_string()));
                assert_eq!(record.v, Some(13280));
                assert_eq!(record.load, Some(true));
                assert_eq!(record.hsds, Some(82));
                assert_eq!(record.or, Some(OffReason::NoInputPower));
                return;
            }
            Err(e) => panic!("Error: {e:?}"),
            _ => (),
        }
    }

    panic!("No record found");
}

#[test]
fn test_valid_smartsolar_mppt75_15_2() {
    let mut reader = TextReader::new();
    for byte in include_bytes!("test_valid_smartsolar_mppt75_15_2.txt") {
        match reader.process_byte(*byte) {
            Ok(Some(record)) => {
                assert_eq!(record.pid, Some(ProductId::SmartSolarMPPT75_15));
                assert_eq!(record.fw, Some("147".to_string()));
                assert_eq!(record.ser, Some("HQ2134AD23G".to_string()));
                assert_eq!(record.v, Some(13270));
                assert_eq!(record.load, Some(true));
                assert_eq!(record.hsds, Some(82));
                assert_eq!(record.or, Some(OffReason::NoInputPower));
                return;
            }
            Err(e) => panic!("Error: {e:?}"),
            _ => (),
        }
    }

    panic!("No record found");
}
