use crate::Message::*;

use crate::Header::Header;

/// Stops any executing actions and put the system in a standby mode.
#[derive(Default)]
pub struct Abort {
    /// IMC Header
    pub header: Header,
}

impl Message for Abort {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Abort { header: hdr };

        msg.get_header()._mgid = 550;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Abort {
            header: Header::new(550),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        550
    }

    fn id(&self) -> u16 {
        550
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
