use crate::Message::*;

use crate::Header::Header;

/// Notification of completion of a Teleoperation maneuver.
#[derive(Default)]
pub struct TeleoperationDone {
    /// IMC Header
    pub header: Header,
}

impl TeleoperationDone {
    pub fn new() -> TeleoperationDone {
        let mut msg = TeleoperationDone {
            header: Header::new(460),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TeleoperationDone {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        460
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
