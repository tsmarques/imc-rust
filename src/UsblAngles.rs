use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// This message contains information, collected using USBL, about the
/// bearing and elevation of a target.
#[derive(Default)]
pub struct UsblAngles {
    /// IMC Header
    pub header: Header,

    /// Target's IMC address.
    pub _target: u16,

    /// Target's bearing.
    pub _bearing: f32,

    /// Target's elevation.
    pub _elevation: f32,
}

impl Message for UsblAngles {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = UsblAngles {
            header: hdr,

            _target: Default::default(),
            _bearing: Default::default(),
            _elevation: Default::default(),
        };

        msg.get_header()._mgid = 890;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = UsblAngles {
            header: Header::new(890),

            _target: Default::default(),
            _bearing: Default::default(),
            _elevation: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        890
    }

    fn id(&self) -> u16 {
        890
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._bearing = Default::default();

        self._elevation = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        10
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._target);
        bfr.put_f32_le(self._bearing);
        bfr.put_f32_le(self._elevation);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
