use crate::packet::ImcError::{BufferTooShort, InvalidCrc, InvalidMessageId, InvalidSync};
use crate::Header::Header;
use crate::Message::Message;
use crate::{
    factory, DUNE_IMC_CONST_MAX_SIZE, DUNE_IMC_CONST_SYNC, IMC_CONST_FOOTER_SIZE,
    IMC_CONST_HEADER_SIZE,
};
use bytes::{Buf, BufMut, IntoBuf};
use crc16::{State, ARC};
use std::borrow::Borrow;

pub mod crc;

pub enum ImcError {
    /// Message is bigger than allowed by IMC
    InvalidMessageSize,
    /// Buffer is to short for remaining bytes
    BufferTooShort,
    /// Invalid synchronization number
    InvalidSync,
    /// Invalid CRC16
    InvalidCrc,
    InvalidMessageId,
}

/// Serialize complete message
// 1. serialize header
// 2. serialize fields
// 3. serialize crc / footer
pub fn serialize(msg: &mut dyn Message, bfr: &mut bytes::BytesMut) -> Result<usize, ImcError> {
    // current buffer position
    let cursor = bfr.len();

    let total = msg.serialization_size();
    if total > DUNE_IMC_CONST_MAX_SIZE {
        return Err(ImcError::InvalidMessageSize);
    }

    if bfr.capacity() < total {
        return Err(ImcError::BufferTooShort);
    }

    // header
    msg.get_header()._size = msg.payload_serialization_size() as u16;
    msg.get_header().serialize(bfr);
    // payload
    msg.serialize_fields(bfr);
    // footer
    let sersize = msg.serialization_size();
    let end = cursor + sersize - IMC_CONST_FOOTER_SIZE as usize;
    let mut crc = crc::compute_from_range(cursor, end, bfr);
    bfr.put_u16_le(crc.get());

    Result::Ok(bfr.len())
}

pub fn deserialize(bfr: &mut dyn bytes::Buf) -> Result<Box<dyn Message>, ImcError> {
    // deserialize header
    let mut hdr: Header = Header::new(0);
    let mut ret = deserializeHeader(&mut hdr, bfr);
    if ret.is_err() {
        return Err(ret.err().unwrap());
    }

    // get header's crc
    let mut crc = ret.ok().unwrap();

    let ser_size = hdr._size as usize;
    let ret = factory::build(hdr);
    if ret.is_none() {
        return Err(InvalidMessageId);
    }

    let mut msg = ret.unwrap();

    // update crc with payload
    crc::update_from_range(&mut crc, 0, ser_size, bfr.bytes());

    msg.deserialize_fields(bfr);

    let read_crc = bfr.get_u16_le();
    if crc.get() != read_crc {
        return Err(InvalidCrc);
    }

    Ok(msg)
}

pub fn deserialize_as<T: Message>(bfr: &mut dyn bytes::Buf) -> Result<Box<T>, ImcError> {
    // deserialize header
    let mut hdr: Header = Header::new(0);
    let mut ret = deserializeHeader(&mut hdr, bfr);
    if ret.is_err() {
        return Err(ret.err().unwrap());
    }

    // get header's crc
    let mut crc = ret.ok().unwrap();

    let ser_size = hdr._size as usize;
    let ret = factory::buildFrom::<T>(hdr);
    if ret.is_none() {
        return Err(InvalidMessageId);
    }

    let mut msg: Box<T> = ret.unwrap();

    // update crc with payload
    crc::update_from_range(&mut crc, 0, ser_size, bfr.bytes());

    msg.deserialize_fields(bfr);

    let read_crc = bfr.get_u16_le();
    if crc.get() != read_crc {
        return Err(InvalidCrc);
    }

    Ok(msg)
}

pub fn deserializeHeader(
    hdr: &mut Header,
    bfr: &mut dyn bytes::Buf,
) -> Result<State<ARC>, ImcError> {
    let remaining = bfr.remaining();
    if remaining < (IMC_CONST_HEADER_SIZE as usize) {
        return Err(BufferTooShort);
    }

    let crc = crc::compute_from_range(0, IMC_CONST_HEADER_SIZE as usize, bfr.bytes());

    hdr._sync = bfr.get_u16_le();
    if hdr._sync != DUNE_IMC_CONST_SYNC {
        return Err(InvalidSync);
    }

    hdr._mgid = bfr.get_u16_le();
    hdr._size = bfr.get_u16_le();
    hdr._timestamp = bfr.get_f64_le();
    hdr._src = bfr.get_u16_le();
    hdr._src_ent = bfr.get_u8();
    hdr._dst = bfr.get_u16_le();
    hdr._dst_ent = bfr.get_u8();

    Ok(crc)
}
