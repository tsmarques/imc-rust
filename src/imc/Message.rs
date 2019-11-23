#![macro_use]

use crate::imc;
use bytes::BufMut;
use crc16::*;

macro_rules! serialize_string {
    ($bfr:expr, $string_var:expr) =>
    {
        $bfr.put_u16_le($string_var.len() as u16);
        $bfr.put_slice($string_var.as_bytes());
    };
}

pub fn serialize_footer(bfr :&mut bytes::BytesMut)
{
    let mut state = State::<ARC>::new();
    state.update(bfr);

    bfr.put_u16_le(state.get());
}

pub trait Message
{
    // Get this messages's static ID
    fn static_id(&self) -> u16;

    // Clear message's fields
    fn clear(&mut self);

    fn fixed_serialization_size(&self) -> usize;
    fn dynamic_serialization_size(&self) -> usize;
    fn serialize(&self, bfr :&mut bytes::BytesMut);

    fn payload_serialization_size(&self) -> usize
    {
        self.fixed_serialization_size() + self.dynamic_serialization_size()
    }

    fn serialization_size(&self) -> usize
    {
        self.payload_serialization_size() +
            imc::IMC_CONST_HEADER_SIZE as usize +
            imc::IMC_CONST_FOOTER_SIZE as usize
    }
}