use crate::Message::*;

use crate::Header::Header;

/// Query the activation/deactivation state of an entity. The
/// recipient shall reply with an EntityActivationState message.
#[derive(Default)]
pub struct QueryEntityActivationState {
    /// IMC Header
    pub header: Header,
}

impl QueryEntityActivationState {
    pub fn new() -> QueryEntityActivationState {
        let mut msg = QueryEntityActivationState {
            header: Header::new(15),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for QueryEntityActivationState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        15
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
