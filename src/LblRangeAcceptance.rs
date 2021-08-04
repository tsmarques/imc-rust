#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum AcceptanceEnum {
    // Accepted
    RR_ACCEPTED = 0,
    // Rejected - Above Threshold
    RR_ABOVE_THRESHOLD = 1,
    // Rejected - Singular Point
    RR_SINGULAR = 2,
    // Rejected - Not Enough Information
    RR_NO_INFO = 3,
    // Rejected - Vehicle At Surface
    RR_AT_SURFACE = 4,
}

impl AcceptanceEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            RR_ACCEPTED => 0,
            RR_ABOVE_THRESHOLD => 1,
            RR_SINGULAR => 2,
            RR_NO_INFO => 3,
            RR_AT_SURFACE => 4,
        }
    }
}

/// Vehicle is using only GPS fix when it is at surface.
#[derive(Default)]
pub struct LblRangeAcceptance {
    /// IMC Header
    pub header: Header,

    /// Identification number of the acoustic transponder from which
    /// the range information was received.
    pub _id: u8,

    /// Distance to the acoustic transponder.
    pub _range: f32,

    /// The filter lacks information to properly use the received LBL range.
    pub _acceptance: u8,
}

impl LblRangeAcceptance {
    pub fn new() -> LblRangeAcceptance {
        let mut msg = LblRangeAcceptance {
            header: Header::new(357),

            _id: Default::default(),
            _range: Default::default(),
            _acceptance: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LblRangeAcceptance {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        357
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._range = Default::default();

        self._acceptance = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._range);
        bfr.put_u8(self._acceptance);
    }
}
