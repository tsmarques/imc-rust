use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum FlagsEnum {
    // Use Speed Reference in meters per second
    FLAG_SPEED_METERS_PS = 0x01,
    // Use Speed Reference in revolutions per minute
    FLAG_SPEED_RPM = 0x02,
    // Use Z Reference as depth
    FLAG_DEPTH = 0x04,
    // Use Z Reference as altitude
    FLAG_ALTITUDE = 0x08,
    // Use Heading Reference
    FLAG_HEADING = 0x10,
    // Use Heading Rate Reference
    FLAG_HEADING_RATE = 0x20,
    // Flag Maneuver Completion
    FLAG_MANDONE = 0x80,
}

impl FlagsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            FLAG_SPEED_METERS_PS => 0x01,
            FLAG_SPEED_RPM => 0x02,
            FLAG_DEPTH => 0x04,
            FLAG_ALTITUDE => 0x08,
            FLAG_HEADING => 0x10,
            FLAG_HEADING_RATE => 0x20,
            FLAG_MANDONE => 0x80,
        }
    }
}

/// Command system to move at a heading rate reference provided in &quot;heading&quot; field in radians/s.
pub struct Command {
    /// IMC Header
    pub header: Header,

    /// Command system to exit maneuver's execution.
    pub _flags: u8,

    /// The value of the desired speed, in the scale specified by the
    /// &quot;flags&quot; field.
    pub _speed: f32,

    /// The value of the desired z reference in meters.
    pub _z: f32,

    /// The value of the desired heading angle, relative to true  north, in radians,
    /// or, the value of the desired heading rate angle, in radians.
    pub _heading: f32,
}

impl Command {
    pub fn new() -> Command {
        let mut msg = Command {
            header: Header::new(497),

            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _heading: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Command {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        497
    }

    fn clear(&mut self) {
        self.header.clear();

        self._flags = Default::default();

        self._speed = Default::default();

        self._z = Default::default();

        self._heading = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        13
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._flags);
        bfr.put_f32_le(self._speed);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._heading);

        serialize_footer(bfr);
    }
}
