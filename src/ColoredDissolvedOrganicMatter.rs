use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

/// Colored Dissolved Organic Matter measurement.
#[derive(Default)]
pub struct ColoredDissolvedOrganicMatter {
    /// IMC Header
    pub header: Header,

    /// Colored Dissolved Organic Matter reading.
    pub _value: f32,
}

impl Message for ColoredDissolvedOrganicMatter {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = ColoredDissolvedOrganicMatter {
            header: Header::new(2003),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = ColoredDissolvedOrganicMatter {
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
        2003
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2003
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_f32_le();
    }
}
