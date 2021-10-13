use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum ValidityEnum {
    // X component is valid
    VAL_VEL_X = 0x01,
    // Y component is valid
    VAL_VEL_Y = 0x02,
    // Z component is valid
    VAL_VEL_Z = 0x04,
}

#[allow(non_camel_case_types)]
pub mod Validity {
    // X component is valid
    pub const VAL_VEL_X: u32 = 0x01;
    // Y component is valid
    pub const VAL_VEL_Y: u32 = 0x02;
    // Z component is valid
    pub const VAL_VEL_Z: u32 = 0x04;
}

/// Vector quantifying the direction and magnitude of the measured
/// velocity relative to the water that a device is exposed to.
#[derive(Default)]
pub struct WaterVelocity {
    /// IMC Header
    pub header: Header,

    /// Each bit of this field represents if a given velocity
    /// component is valid.
    pub _validity: u8,

    /// X component.
    pub _x: f64,

    /// Y component.
    pub _y: f64,

    /// Z component.
    pub _z: f64,
}

impl Message for WaterVelocity {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = WaterVelocity {
            header: Header::new(260),

            _validity: Default::default(),
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
        let msg = WaterVelocity {
            header: hdr,

            _validity: Default::default(),
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
        260
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        260
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._validity = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        25
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._validity);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u8();
        self._x = bfr.get_f64_le();
        self._y = bfr.get_f64_le();
        self._z = bfr.get_f64_le();

        Ok(())
    }
}
