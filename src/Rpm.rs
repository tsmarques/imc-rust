use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Number of revolutions per minute.
#[derive(Default)]
pub struct Rpm {
    /// IMC Header
    pub header: Header,

    /// Number of revolutions per minute.
    pub _value: i16,
}

impl Message for Rpm {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Rpm {
            header: Header::new(250),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Rpm {
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
        250
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        250
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
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_i16_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_i16_le();
    }
}
