use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::MessageGroup::Maneuver;

/// message-group: Maneuver
impl Maneuver for Teleoperation {}

/// The Teleoperation Maneuver lets the vehicle be controlled by an
/// external human operator.
/// message-group: Maneuver
pub struct Teleoperation {
    /// IMC Header
    pub header: Header,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Teleoperation {
    pub fn new() -> Teleoperation {
        let mut msg = Teleoperation {
            header: Header::new(452),

            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Teleoperation {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        452
    }

    fn clear(&mut self) {
        self.header.clear();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._custom.as_bytes());

        serialize_footer(bfr);
    }
}
