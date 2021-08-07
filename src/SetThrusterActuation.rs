use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Actuate directly on a thruster.
#[derive(Default)]
pub struct SetThrusterActuation {
    /// IMC Header
    pub header: Header,

    /// The identification number of the destination thruster.
    pub _id: u8,

    /// Actuation magnitude.
    pub _value: f32,
}

impl Message for SetThrusterActuation {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = SetThrusterActuation {
            header: hdr,

            _id: Default::default(),
            _value: Default::default(),
        };

        msg.get_header()._mgid = 301;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = SetThrusterActuation {
            header: Header::new(301),

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
        301
    }

    fn id(&self) -> u16 {
        301
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
