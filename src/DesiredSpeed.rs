use crate::Message::*;

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

impl Message for DesiredSpeed {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = DesiredSpeed {
            header: hdr,

            _value: Default::default(),
            _speed_units: Default::default(),
        };

        msg.get_header()._mgid = 402;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = DesiredSpeed {
            header: Header::new(402),

            _value: Default::default(),
            _speed_units: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        402
    }

    fn id(&self) -> u16 {
        402
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
        bfr.put_u8(self._speed_units);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
