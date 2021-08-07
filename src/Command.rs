use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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
#[derive(Default)]
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

impl Message for Command {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Command {
            header: hdr,

            _flags: Default::default(),
            _speed: Default::default(),
            _z: Default::default(),
            _heading: Default::default(),
        };

        msg.get_header()._mgid = 497;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
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

    fn static_id() -> u16
    where
        Self: Sized,
    {
        497
    }

    fn id(&self) -> u16 {
        497
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._flags);
        bfr.put_f32_le(self._speed);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._heading);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
