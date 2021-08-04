use crate::Message::*;

use crate::Header::Header;

/// Command used to stop currently executing maneuver.
#[derive(Default)]
pub struct StopManeuver {
    /// IMC Header
    pub header: Header,
}

impl StopManeuver {
    pub fn new() -> StopManeuver {
        let mut msg = StopManeuver {
            header: Header::new(468),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for StopManeuver {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        468
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
