use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum ReasonEnum {
    // Above Threshold
    RR_ABOVE_THRESHOLD = 0,
    // Invalid Fix
    RR_INVALID = 1,
    // Above Maximum HDOP
    RR_ABOVE_MAX_HDOP = 2,
    // Above Maximum HACC
    RR_ABOVE_MAX_HACC = 3,
    // Lost Validity Bit
    RR_LOST_VAL_BIT = 4,
}

impl ReasonEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            RR_ABOVE_THRESHOLD => 0,
            RR_INVALID => 1,
            RR_ABOVE_MAX_HDOP => 2,
            RR_ABOVE_MAX_HACC => 3,
            RR_LOST_VAL_BIT => 4,
        }
    }
}

/// Lost one of the validity bits between consecutive GPS fixes.
#[derive(Default)]
pub struct GpsFixRejection {
    /// IMC Header
    pub header: Header,

    /// UTC time of the rejected GPS fix measured in seconds since
    /// 00:00:00 (midnight).
    pub _utc_time: f32,

    /// Above maximum horizontal accuracy index.
    pub _reason: u8,
}

impl Message for GpsFixRejection {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = GpsFixRejection {
            header: Header::new(356),

            _utc_time: Default::default(),
            _reason: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = GpsFixRejection {
            header: hdr,

            _utc_time: Default::default(),
            _reason: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        356
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        356
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._utc_time = Default::default();

        self._reason = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._utc_time);
        bfr.put_u8(self._reason);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._utc_time = bfr.get_f32_le();

        self._reason = bfr.get_u8();
    }
}
