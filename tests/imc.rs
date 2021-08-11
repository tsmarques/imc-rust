use bytes::{BufMut, IntoBuf};
use crc16::{State, ARC};
use imc::packet::{crc, deserializeHeader};
use imc::EstimatedState::EstimatedState;
use imc::Header::Header;
use imc::LoggingControl::{ControlOperationEnum, LoggingControl};
use imc::Message::Message;
use imc::{IMC_CONST_FOOTER_SIZE, IMC_CONST_HEADER_SIZE};

#[test]
fn buffer_write() {
    let mut bfr = bytes::BytesMut::with_capacity(1024);
    bfr.put("IMC RUST");

    assert_eq!(bfr[0], b'I');
    // make sure it doesn't advance cursor
    assert_ne!(bfr[0], b'M');
    assert_eq!(bfr.len(), 8);

    bfr.put("look at me testing");
    assert_eq!(bfr[0], b'I');
    assert_eq!(bfr[8], b'l');
    assert_eq!(bfr[8..].len(), b"look at me testing".len());
}

#[test]
fn imc_crc() {
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

    let crc = crc::compute_from_range(
        0,
        lc.serialization_size() - IMC_CONST_FOOTER_SIZE as usize,
        &mut bfr,
    );

    assert_eq!(crc.get(), 1427_u16);
}

#[test]
fn header_serialization() {
    let mut hdr: Header = Header::new(1_u16);
    hdr._timestamp = 0.143432;
    hdr._size = 42;
    hdr._src = 205;
    hdr._src_ent = 2;
    hdr._dst = 300;
    hdr._dst_ent = 10;

    let mut bfr: bytes::BytesMut = bytes::BytesMut::with_capacity(IMC_CONST_HEADER_SIZE as usize);
    hdr.serialize(&mut bfr);

    let mut hdr2: Header = Header::new(0);
    let inbfr = bytes::Bytes::from(bfr);
    let ret = deserializeHeader(&mut hdr2, &mut inbfr.into_buf());

    assert!(ret.is_ok());
    assert_eq!(hdr, hdr2);
}

#[test]
fn deserialize_as() {
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

    let inbfr = bytes::Bytes::from(bfr);
    let ret = imc::packet::deserialize_as::<LoggingControl>(&mut inbfr.into_buf());
    assert!(ret.is_ok());

    let mut lc2 = ret.ok().unwrap();
    assert_eq!(lc.get_header(), lc2.get_header());
    assert_eq!(lc._op, lc2._op);
    assert_eq!(lc._name, lc2._name);
}

#[test]
fn generic_deserialize() {
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

    let inbfr = bytes::Bytes::from(bfr);
    let ret = imc::packet::deserialize(&mut inbfr.into_buf());
    assert!(ret.is_ok());

    let mut lc2 = ret.ok().unwrap();
    assert_eq!(lc.get_header(), lc2.get_header());
}
