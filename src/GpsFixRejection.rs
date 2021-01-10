use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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
    pub fn as_primitive(&self) -> u32 {
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
pub struct GpsFixRejection {
    /// IMC Header
    pub header: Header,

    /// UTC time of the rejected GPS fix measured in seconds since
    /// 00:00:00 (midnight).
    pub _utc_time: f32,

    /// Above maximum horizontal accuracy index.
    pub _reason: u8,
}

impl GpsFixRejection {
    pub fn new() -> GpsFixRejection {
        let mut msg = GpsFixRejection {
            header: Header::new(356),

            _utc_time: Default::default(),
            _reason: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GpsFixRejection {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        356
    }

    fn clear(&mut self) {
        self.header.clear();

        self._utc_time = Default::default();

        self._reason = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._utc_time);
        bfr.put_u8(self._reason);

        serialize_footer(bfr);
    }
}
