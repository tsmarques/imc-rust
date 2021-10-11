use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

/// Initiate overall calibration of a vehicle.
#[derive(Default)]
pub struct Calibration {
    /// IMC Header
    pub header: Header,

    /// Duration of calibration.
    pub _duration: u16,
}

impl Message for Calibration {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Calibration {
            header: Header::new(506),

            _duration: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Calibration {
            header: hdr,

            _duration: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        506
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        506
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._duration = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._duration);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._duration = bfr.get_u16_le();
    }
}
