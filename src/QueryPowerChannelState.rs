use crate::Message::*;

use crate::Header::Header;

/// Request the state of power channels.
#[derive(Default)]
pub struct QueryPowerChannelState {
    /// IMC Header
    pub header: Header,
}

impl QueryPowerChannelState {
    pub fn new() -> QueryPowerChannelState {
        let mut msg = QueryPowerChannelState {
            header: Header::new(310),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for QueryPowerChannelState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        310
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
