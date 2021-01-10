use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredHeading {}

/// Desired Heading angle reference value for the control layer.
/// message-group: ControlCommand
pub struct DesiredHeading {
    /// IMC Header
    pub header: Header,

    /// The value of the desired heading angle, relative to true
    /// north, in radians.
    pub _value: f64,
}

impl DesiredHeading {
    pub fn new() -> DesiredHeading {
        let mut msg = DesiredHeading {
            header: Header::new(400),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredHeading {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        400
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._value);

        serialize_footer(bfr);
    }
}
