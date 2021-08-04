use crate::Message::*;

use crate::Header::Header;

/// Hardware pulse detection.
#[derive(Default)]
pub struct Pulse {
    /// IMC Header
    pub header: Header,
}

impl Pulse {
    pub fn new() -> Pulse {
        let mut msg = Pulse {
            header: Header::new(277),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Pulse {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        277
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
