#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredSpeed {}

/// Desired Speed reference value for the control layer.
/// message-group: ControlCommand
#[derive(Default)]
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
            header: Header::new(402),

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
        402
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();

        self._speed_units = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
        bfr.put_u8(self._speed_units);
    }
}
