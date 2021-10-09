use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// Verbatim representation of device data in plain text format.
#[derive(Default)]
pub struct DevDataText {
    /// IMC Header
    pub header: Header,

    /// Plain text data as extracted directly from the device.
    pub _value: String,
}

impl Message for DevDataText {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DevDataText {
            header: Header::new(273),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DevDataText {
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
        273
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        273
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
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._value.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._value);
    }
}
