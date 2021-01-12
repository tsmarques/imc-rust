use crate::Header;
use crate::{Message::Message, DUNE_IMC_CONST_MAX_SIZE};

pub enum PacketError {
    InvalidMessageSize,
    BufferTooShort,
    InvalidSync,
    InvalidCrc,
    InvalidMessageId,
}

pub fn serialize(msg: &dyn Message, bfr: &mut bytes::BytesMut) -> Result<usize, PacketError> {
    let total = msg.serialization_size();
    if total > DUNE_IMC_CONST_MAX_SIZE {
        return Err(PacketError::InvalidMessageSize);
    }

    if bfr.capacity() < total {
        return Err(PacketError::BufferTooShort);
    }

    msg.serialize(bfr);
    Result::Ok(total)
}

pub fn deserialize() {
    unimplemented!()
}
