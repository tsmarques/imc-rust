use crate::DUNE_IMC_CONST_MAX_SIZE;
use crate::Message::Message;
use crc16::{State, ARC};
use bytes::BufMut;

pub enum PacketError {
    InvalidMessageSize,
    BufferTooShort,
    InvalidSync,
    InvalidCrc,
    InvalidMessageId,
}

/// Serialize complete message
// 1. serialize header
// 2. serialize fields
// 3. serialize crc / footer
pub fn serialize(msg: &mut dyn Message, bfr: &mut bytes::BytesMut) -> Result<usize, PacketError> {
    let total = msg.serialization_size();
    if total > DUNE_IMC_CONST_MAX_SIZE {
        return Err(PacketError::InvalidMessageSize);
    }

    if bfr.capacity() < total {
        return Err(PacketError::BufferTooShort);
    }

    // header
    msg.get_header().serialize(bfr);
    // payload
    msg.serialize_fields(bfr);
    // footer
    let mut state = State::<ARC>::new();
    state.update(bfr);
    bfr.put_u16_le(state.get());

    Result::Ok(bfr.len())
}

pub fn deserialize() {
    unimplemented!()
}