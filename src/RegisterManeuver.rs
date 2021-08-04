use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Command used to indicate maneuver can be executed in the
/// vehicle.
#[derive(Default)]
pub struct RegisterManeuver {
    /// IMC Header
    pub header: Header,

    /// IMC serialization ID of maneuver type.
    pub _mid: u16,
}

impl RegisterManeuver {
    pub fn new() -> RegisterManeuver {
        let mut msg = RegisterManeuver {
            header: Header::new(469),

            _mid: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RegisterManeuver {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        469
    }

    fn clear(&mut self) {
        self.header.clear();

        self._mid = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._mid);
    }
}
