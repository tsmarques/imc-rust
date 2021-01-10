use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredPitch {}

/// Desired Pitch angle reference value for the control layer.
/// message-group: ControlCommand
pub struct DesiredPitch {
    /// IMC Header
    pub header: Header,

    /// The value of the desired pitch angle.
    pub _value: f64,
}

impl DesiredPitch {
    pub fn new() -> DesiredPitch {
        let mut msg = DesiredPitch {
            header: Header::new(404),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredPitch {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        404
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
