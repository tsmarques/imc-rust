use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DesiredPitch {
            header: Header::new(404),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DesiredPitch {
            header: hdr,

            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        404
    }

    #[inline(always)]
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

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_f64_le();
    }
}
