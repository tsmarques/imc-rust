use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Command used to stop currently executing maneuver.
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
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_footer(bfr);
    }
}
