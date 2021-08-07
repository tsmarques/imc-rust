use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of pH.
#[derive(Default)]
pub struct PH {
    /// IMC Header
    pub header: Header,

    /// The value of the pH as measured by the sensor.
    pub _value: f32,
}

impl Message for PH {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = PH {
            header: hdr,

            _value: Default::default(),
        };

        msg.get_header()._mgid = 298;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = PH {
            header: Header::new(298),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        298
    }

    fn id(&self) -> u16 {
        298
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
