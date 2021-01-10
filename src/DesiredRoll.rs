use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredRoll {}

/// Desired Roll angle reference value for the control layer.
/// message-group: ControlCommand
pub struct DesiredRoll {
    /// IMC Header
    pub header: Header,

    /// The value of the desired roll angle.
    pub _value: f64,
}

impl DesiredRoll {
    pub fn new() -> DesiredRoll {
        let mut msg = DesiredRoll {
            header: Header::new(403),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredRoll {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        403
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
