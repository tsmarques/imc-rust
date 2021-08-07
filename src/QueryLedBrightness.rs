use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Query the brightness of an LED (Light-Emitting Diode). The
/// recipient of this message shall reply with 'LedBrightness'.
#[derive(Default)]
pub struct QueryLedBrightness {
    /// IMC Header
    pub header: Header,

    /// LED name.
    pub _name: String,
}

impl Message for QueryLedBrightness {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = QueryLedBrightness {
            header: hdr,

            _name: Default::default(),
        };

        msg.get_header()._mgid = 313;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = QueryLedBrightness {
            header: Header::new(313),

            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        313
    }

    fn id(&self) -> u16 {
        313
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
