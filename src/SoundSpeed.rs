use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Sound Speed report.
#[derive(Default)]
pub struct SoundSpeed {
    /// IMC Header
    pub header: Header,

    /// Estimated sound speed. Negative values denote invalid estimates.
    pub _value: f32,
}

impl Message for SoundSpeed {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = SoundSpeed {
            header: hdr,

            _value: Default::default(),
        };

        msg.get_header()._mgid = 267;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = SoundSpeed {
            header: Header::new(267),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        267
    }

    fn id(&self) -> u16 {
        267
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
