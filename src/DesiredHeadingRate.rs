use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Desired Heading Rate speed reference value for the control layer.
#[derive(Default)]
pub struct DesiredHeadingRate {
    /// IMC Header
    pub header: Header,

    /// The value of the desired heading rate speed in radians per
    /// second.
    pub _value: f64,
}

impl Message for DesiredHeadingRate {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DesiredHeadingRate {
            header: Header::new(408),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DesiredHeadingRate {
            header: hdr,

            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        408
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        408
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_f64_le();
    }
}
