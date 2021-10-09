use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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

impl Message for LblRangeAcceptance {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LblRangeAcceptance {
            header: Header::new(357),

            _id: Default::default(),
            _range: Default::default(),
            _acceptance: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LblRangeAcceptance {
            header: hdr,

            _id: Default::default(),
            _range: Default::default(),
            _acceptance: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        357
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        357
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._range = Default::default();

        self._acceptance = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._range);
        bfr.put_u8(self._acceptance);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._id = bfr.get_u8();

        self._range = bfr.get_f32_le();

        self._acceptance = bfr.get_u8();
    }
}
