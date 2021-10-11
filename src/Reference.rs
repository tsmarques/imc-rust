use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::DesiredZ::DesiredZ;

use crate::DesiredSpeed::DesiredSpeed;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Use Location Reference
    FLAG_LOCATION = 0x01,
    // Use Speed Reference
    FLAG_SPEED = 0x02,
    // Use Z Reference
    FLAG_Z = 0x04,
    // Use Radius Reference
    FLAG_RADIUS = 0x08,
    // Use this Reference as Start Position for PathControler
    FLAG_START_POINT = 0x10,
    // Use Current Position as Start Position for PathControler
    FLAG_DIRECT = 0x20,
    // Flag Maneuver Completion
    FLAG_MANDONE = 0x80,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            FLAG_LOCATION => 0x01,
            FLAG_SPEED => 0x02,
            FLAG_Z => 0x04,
            FLAG_RADIUS => 0x08,
            FLAG_START_POINT => 0x10,
            FLAG_DIRECT => 0x20,
            FLAG_MANDONE => 0x80,
        }
    }
}

#[derive(Default)]
pub struct Reference {
    /// IMC Header
    pub header: Header,

    pub _flags: u8,

    pub _speed: Option<DesiredSpeed>,

    pub _z: Option<DesiredZ>,

    pub _lat: f64,

    pub _lon: f64,

    pub _radius: f32,
}

impl Message for Reference {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Reference {
            header: Header::new(479),

            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _radius: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Reference {
            header: hdr,

            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _radius: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        479
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        479
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._flags = Default::default();

        self._speed = Default::default();

        self._z = Default::default();

        self._lat = Default::default();

        self._lon = Default::default();

        self._radius = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        21
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        inline_message_serialization_size!(dyn_size, self._speed);

        inline_message_serialization_size!(dyn_size, self._z);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._flags);
        serialize_inline_message!(bfr, self._speed);
        serialize_inline_message!(bfr, self._z);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._radius);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._flags = bfr.get_u8();

        self._speed = deserialize_inline_as::<DesiredSpeed>(bfr).ok();

        self._z = deserialize_inline_as::<DesiredZ>(bfr).ok();

        self._lat = bfr.get_f64_le();

        self._lon = bfr.get_f64_le();

        self._radius = bfr.get_f32_le();

        Ok(())
    }
}
