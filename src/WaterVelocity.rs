use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum ValidityEnum {
    // X component is valid
    VAL_VEL_X = 0x01,
    // Y component is valid
    VAL_VEL_Y = 0x02,
    // Z component is valid
    VAL_VEL_Z = 0x04,
}

impl ValidityEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            VAL_VEL_X => 0x01,
            VAL_VEL_Y => 0x02,
            VAL_VEL_Z => 0x04,
        }
    }
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

impl WaterVelocity {
    pub fn new() -> WaterVelocity {
        let mut msg = WaterVelocity {
            header: Header::new(260),

            _validity: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for WaterVelocity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        260
    }

    fn clear(&mut self) {
        self.header.clear();

        self._validity = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

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
}
