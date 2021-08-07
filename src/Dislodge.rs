use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::MessageGroup::Maneuver;

#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    // Let the vehicle decide
    DIR_AUTO = 0,
    // Attempt to move forward
    DIR_FORWARD = 1,
    // Attempt to move backward
    DIR_BACKWARD = 2,
}

impl DirectionEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            DIR_AUTO => 0,
            DIR_FORWARD => 1,
            DIR_BACKWARD => 2,
        }
    }
}

/// message-group: Maneuver
impl Maneuver for Dislodge {}

/// A &quot;Dislodge&quot; is a maneuver ordering the vehicle to attempt a
/// series of thruster operations that will hopefully get it
/// unstuck from an entangled condition.
///  
/// Parameters are RPMs for the motor when attempting dislodge and
/// and a flag specifying whether the thrust burst should be attempted
/// forward, backward or auto (letting the vehicle decide).
/// message-group: Maneuver
#[derive(Default)]
pub struct Dislodge {
    /// IMC Header
    pub header: Header,

    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,

    /// Maneuver RPM reference.
    pub _rpm: f32,

    /// Direction to which the vehicle should attempt to unstuck.
    pub _direction: u8,

    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Dislodge {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Dislodge {
            header: hdr,

            _timeout: Default::default(),
            _rpm: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg.get_header()._mgid = 483;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Dislodge {
            header: Header::new(483),

            _timeout: Default::default(),
            _rpm: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        483
    }

    fn id(&self) -> u16 {
        483
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeout = Default::default();

        self._rpm = Default::default();

        self._direction = Default::default();

        self._custom = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        7
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_f32_le(self._rpm);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
