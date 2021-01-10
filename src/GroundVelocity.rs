use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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
/// velocity relative to the ground that a device is exposed to.
pub struct GroundVelocity {
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

impl GroundVelocity {
    pub fn new() -> GroundVelocity {
        let mut msg = GroundVelocity {
            header: Header::new(259),

            _validity: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GroundVelocity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        259
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
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._validity);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);

        serialize_footer(bfr);
    }
}
