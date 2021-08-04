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

impl Heartbeat {
    pub fn new() -> Heartbeat {
        let mut msg = Heartbeat {
            header: Header::new(150),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Heartbeat {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        150
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
