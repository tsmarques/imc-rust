use crate::Message::*;

use crate::Header::Header;

/// Stops any executing actions and put the system in a standby mode.
#[derive(Default)]
pub struct Abort {
    /// IMC Header
    pub header: Header,
}

impl Abort {
    pub fn new() -> Abort {
        let mut msg = Abort {
            header: Header::new(550),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Abort {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        550
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
}
