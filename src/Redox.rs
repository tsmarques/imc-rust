use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of Redox Potential.
#[derive(Default)]
pub struct Redox {
    /// IMC Header
    pub header: Header,

    /// The value of the Redox as measured by the sensor.
    pub _value: f32,
}

impl Message for Redox {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Redox {
            header: Header::new(299),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Redox {
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
        299
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        299
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
