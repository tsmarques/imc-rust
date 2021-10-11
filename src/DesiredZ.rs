use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::ControlCommand;

use crate::packet::ImcError;
use crate::packet::*;

/// message-group: ControlCommand
// impl ControlCommand for DesiredZ { }

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

impl Message for DesiredZ {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DesiredZ {
            header: Header::new(401),

            _value: Default::default(),
            _z_units: 0_u8,
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DesiredZ {
            header: hdr,

            _value: Default::default(),
            _z_units: 0_u8,
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        401
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        401
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
        self._z_units = Default::default();
    }

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();
        self._z_units = bfr.get_u8();

        Ok(())
    }
}
