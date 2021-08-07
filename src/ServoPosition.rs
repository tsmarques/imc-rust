use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Actual position of a servo.
#[derive(Default)]
pub struct ServoPosition {
    /// IMC Header
    pub header: Header,

    /// Servo identifier.
    pub _id: u8,

    /// Value of the servo position.
    pub _value: f32,
}

impl Message for ServoPosition {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ServoPosition {
            header: hdr,

            _id: Default::default(),
            _value: Default::default(),
        };

        msg.get_header()._mgid = 281;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ServoPosition {
            header: Header::new(281),

            _id: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        281
    }

    fn id(&self) -> u16 {
        281
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
