use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Air Saturation measurement.
#[derive(Default)]
pub struct AirSaturation {
    /// IMC Header
    pub header: Header,

    /// Air Saturation reading.
    pub _value: f32,
}

impl Message for AirSaturation {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AirSaturation {
            header: Header::new(296),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AirSaturation {
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
        296
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        296
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
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();

        Ok(())
    }
}
