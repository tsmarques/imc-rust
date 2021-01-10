#![macro_use]

use crate::Header::Header;
use crate::{IMC_CONST_FOOTER_SIZE, IMC_CONST_HEADER_SIZE};
use bytes::BufMut;
use crc16::{State, ARC};

macro_rules! serialize_bytes {
    ($bfr:expr, $bytes_slice:expr) => {
        $bfr.put_u16_le($bytes_slice.len() as u16);
        $bfr.put_slice($bytes_slice);
    };
}

pub fn serialize_footer(bfr: &mut bytes::BytesMut) {
    let mut state = State::<ARC>::new();
    state.update(bfr);

    bfr.put_u16_le(state.get());
}

pub trait Message {
    fn get_header(&mut self) -> &mut Header;

    fn set_size(&mut self, size: u16) {
        self.get_header()._size = size
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
    }

    // Get this messages's static ID
    fn static_id(&self) -> u16;

    // Clear message's fields
    fn clear(&mut self);

    fn fixed_serialization_size(&self) -> usize;
    fn dynamic_serialization_size(&self) -> usize;
    fn serialize(&self, bfr: &mut bytes::BytesMut);

    fn payload_serialization_size(&self) -> usize {
        self.fixed_serialization_size() + self.dynamic_serialization_size()
    }

    fn serialization_size(&self) -> usize {
        self.payload_serialization_size()
            + IMC_CONST_HEADER_SIZE as usize
            + IMC_CONST_FOOTER_SIZE as usize
    }
}
