use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// Brightness value of an LED (Light-Emitting Diode).
#[derive(Default)]
pub struct LedBrightness {
    /// IMC Header
    pub header: Header,

    /// LED name.
    pub _name: String,

    /// Brightness value.
    pub _value: u8,
}

impl Message for LedBrightness {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = LedBrightness {
            header: Header::new(312),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = LedBrightness {
            header: hdr,

            _name: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        312
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        312
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);

        self._value = bfr.get_u8();
    }
}
