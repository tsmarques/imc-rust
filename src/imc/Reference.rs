use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::DesiredSpeed::DesiredSpeed;

use crate::imc::DesiredZ::DesiredZ;

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
    pub fn as_primitive(&self) -> u32 {
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

pub struct Reference {
    /// IMC Header
    pub header: Header,

    pub _flags: u8,

    pub _speed: Option<Box<DesiredSpeed>>,

    pub _z: Option<Box<DesiredZ>>,

    pub _lat: f64,

    pub _lon: f64,

    pub _radius: f32,
}

impl Reference {
    pub fn new() -> Reference {
        let mut msg = Reference {
            header: Header::new(479),

            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _radius: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Reference {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        479
    }

    fn clear(&mut self) {
        self.header.clear();

        self._flags = Default::default();

        match &mut self._speed {
            Some(field) => field.clear(),

            None => {}
        }

        match &mut self._z {
            Some(field) => field.clear(),

            None => {}
        }

        self._lat = Default::default();

        self._lon = Default::default();

        self._radius = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        21
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        match &self._speed {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        match &self._z {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._flags);
        match &self._speed {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        match &self._z {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._radius);

        serialize_footer(bfr);
    }
}
