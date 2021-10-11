use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// This message contains information, collected using USBL, about a
/// target's position.
#[derive(Default)]
pub struct UsblPosition {
    /// IMC Header
    pub header: Header,

    /// Target's IMC address.
    pub _target: u16,

    /// X coordinate of the target in the local device's reference frame.
    pub _x: f32,

    /// Y coordinate of the target in the local device's reference frame.
    pub _y: f32,

    /// Z coordinate of the target in the local device's reference frame.
    pub _z: f32,
}

impl Message for UsblPosition {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UsblPosition {
            header: Header::new(891),

            _target: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UsblPosition {
            header: hdr,

            _target: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        891
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        891
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        14
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._target);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._target = bfr.get_u16_le();

        self._x = bfr.get_f32_le();

        self._y = bfr.get_f32_le();

        self._z = bfr.get_f32_le();

        Ok(())
    }
}
