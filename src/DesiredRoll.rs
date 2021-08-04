use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredRoll {}

/// Desired Roll angle reference value for the control layer.
/// message-group: ControlCommand
#[derive(Default)]
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
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
    }
}
