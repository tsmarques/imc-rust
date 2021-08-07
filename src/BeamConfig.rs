use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Beam configuration of the device.
#[derive(Default)]
pub struct BeamConfig {
    /// IMC Header
    pub header: Header,

    /// Beam width of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_width: f32,

    /// Beam height of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_height: f32,
}

impl Message for BeamConfig {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = BeamConfig {
            header: hdr,

            _beam_width: Default::default(),
            _beam_height: Default::default(),
        };

        msg.get_header()._mgid = 283;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = BeamConfig {
            header: Header::new(283),

            _beam_width: Default::default(),
            _beam_height: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        283
    }

    fn id(&self) -> u16 {
        283
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._beam_width = Default::default();

        self._beam_height = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._beam_width);
        bfr.put_f32_le(self._beam_height);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
