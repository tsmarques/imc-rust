use crate::Message::*;

use crate::Header::Header;

/// The Heartbeat message is used to inform other modules that the
/// sending entity's system is running normally and communications
/// are alive.
#[derive(Default)]
pub struct Heartbeat {
    /// IMC Header
    pub header: Header,
}

impl Message for Heartbeat {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Heartbeat { header: hdr };

        msg.get_header()._mgid = 150;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Heartbeat {
            header: Header::new(150),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        150
    }

    fn id(&self) -> u16 {
        150
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
