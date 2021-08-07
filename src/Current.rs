use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of electrical current.
#[derive(Default)]
pub struct Current {
    /// IMC Header
    pub header: Header,

    /// The value of the internal electrical current as measured by
    /// the sensor.
    pub _value: f32,
}

impl Message for Current {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Current {
            header: hdr,

            _value: Default::default(),
        };

        msg.get_header()._mgid = 252;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Current {
            header: Header::new(252),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        252
    }

    fn id(&self) -> u16 {
        252
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
