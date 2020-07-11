use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

use crate::imc::MessageGroup::ControlCommand;

const c_msg_id: u16 = 402;

/// message-group: ControlCommand
impl ControlCommand for DesiredSpeed {}

/// Desired Speed reference value for the control layer.
/// message-group: ControlCommand
pub struct DesiredSpeed {
    /// IMC Header
    pub header: Header,

    /// The value of the desired speed, in the scale specified by the
    /// &quot;Speed Units&quot; field.
    pub _value: f64,

    /// Indicates the units used for the speed value.
    pub _speed_units: u8,
}

impl DesiredSpeed {
    pub fn new() -> DesiredSpeed {
        let mut msg = DesiredSpeed {
            header: Header::new(c_msg_id),

            _value: Default::default(),
            _speed_units: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredSpeed {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();

        self._speed_units = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._value);
        bfr.put_u8(self._speed_units);

        serialize_footer(bfr);
    }
}
