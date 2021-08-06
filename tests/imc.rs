use imc::EstimatedState::EstimatedState;
use imc::LoggingControl::{ControlOperationEnum, LoggingControl};
use imc::Message::Message;

#[test]
fn logging_control_0() {
    let mut lc = LoggingControl::new();
    lc.set_timestamp_secs(0.23424);
    lc.set_source(765);
    lc.set_source_ent(230);
    lc.set_destination(57);
    lc.set_destination_ent(32);
    lc._name = String::from("20210707_IMC_RUST_TEST");
    lc._op = ControlOperationEnum::COP_REQUEST_START.value();

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(lc.serialization_size());

    let ret = imc::packet::serialize(&mut lc, &mut bfr);
    assert!(ret.is_ok());
    assert_eq!(ret.ok().unwrap(), lc.serialization_size());
}
