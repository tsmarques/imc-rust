use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

/// message-group: ControlCommand
impl ControlCommand for DesiredPitch {}

/// Desired Pitch angle reference value for the control layer.
/// message-group: ControlCommand
#[derive(Default)]
pub struct DesiredPitch {
    /// IMC Header
    pub header: Header,

    /// The value of the desired pitch angle.
    pub _value: f64,
}

impl Message for DesiredPitch {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = DesiredPitch {
            header: hdr,

            _value: Default::default(),
        };

        msg.get_header()._mgid = 404;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = DesiredPitch {
            header: Header::new(404),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        404
    }

    fn id(&self) -> u16 {
        404
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
