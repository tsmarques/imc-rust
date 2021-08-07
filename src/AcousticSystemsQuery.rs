use crate::Message::*;

use crate::Header::Header;

/// Request a list of known underwater acoustic systems. The
/// recipient of this message shall reply with an AcousticSystems
/// message.
#[derive(Default)]
pub struct AcousticSystemsQuery {
    /// IMC Header
    pub header: Header,
}

impl Message for AcousticSystemsQuery {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = AcousticSystemsQuery { header: hdr };

        msg.get_header()._mgid = 212;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = AcousticSystemsQuery {
            header: Header::new(212),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        212
    }

    fn id(&self) -> u16 {
        212
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {}

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
