use crate::Message::*;

use crate::Header::Header;

/// Request entities to report their state. Entities should respond
/// by issuing an appropriate EntityState message.
#[derive(Default)]
pub struct QueryEntityState {
    /// IMC Header
    pub header: Header,
}

impl QueryEntityState {
    pub fn new() -> QueryEntityState {
        let mut msg = QueryEntityState {
            header: Header::new(2),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for QueryEntityState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2
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
