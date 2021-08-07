use crate::Message::*;

use crate::Header::Header;

/// Command used to stop currently executing maneuver.
#[derive(Default)]
pub struct StopManeuver {
    /// IMC Header
    pub header: Header,
}

impl Message for StopManeuver {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = StopManeuver { header: hdr };

        msg.get_header()._mgid = 468;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = StopManeuver {
            header: Header::new(468),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        468
    }

    fn id(&self) -> u16 {
        468
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
