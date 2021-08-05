use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredZ {}

/// Desired Z reference value for the control layer.
/// message-group: ControlCommand
#[derive(Default)]
pub struct DesiredZ {
    /// IMC Header
    pub header: Header,

    /// The value of the desired z reference in meters.
    pub _value: f32,

    /// Units of the z reference.
    pub _z_units: u8,
}

impl DesiredZ {
    pub fn new() -> DesiredZ {
        let mut msg = DesiredZ {
            header: Header::new(401),

            _value: Default::default(),
            _z_units: 0_u8,
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredZ {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        401
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();

        self._z_units = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._z_units);
    }
}
