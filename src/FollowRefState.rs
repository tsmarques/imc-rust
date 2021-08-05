use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::Reference::Reference;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Waiting for first reference
    FR_WAIT = 1,
    // Going towards received reference
    FR_GOTO = 2,
    // Loitering after arriving at the reference
    FR_LOITER = 3,
    // Hovering after arriving at the reference
    FR_HOVER = 4,
    // Moving in z after arriving at the target cylinder
    FR_ELEVATOR = 5,
    // Controlling system timed out
    FR_TIMEOUT = 6,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            FR_WAIT => 1,
            FR_GOTO => 2,
            FR_LOITER => 3,
            FR_HOVER => 4,
            FR_ELEVATOR => 5,
            FR_TIMEOUT => 6,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum ProximityEnum {
    // Far from the destination
    PROX_FAR = 0x01,
    // Near in the horizontal plane
    PROX_XY_NEAR = 0x02,
    // Near in the vertical plane
    PROX_Z_NEAR = 0x04,
    // Unreachable in the horizontal plane
    PROX_XY_UNREACHABLE = 0x08,
    // Unreachable in the vertical plane
    PROX_Z_UNREACHABLE = 0x10,
}

impl ProximityEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            PROX_FAR => 0x01,
            PROX_XY_NEAR => 0x02,
            PROX_Z_NEAR => 0x04,
            PROX_XY_UNREACHABLE => 0x08,
            PROX_Z_UNREACHABLE => 0x10,
        }
    }
}

#[derive(Default)]
pub struct FollowRefState {
    /// IMC Header
    pub header: Header,

    /// The IMC identifier of the source system that is allowed to control the vehicle.
    /// If the value ''0xFFFF'' is used, any system is allowed to command references.
    pub _control_src: u16,

    /// The entity identifier of the entity that is allowed to control the vehicle.
    /// If the value ''0xFF'' is used, any entity is allowed to command references.
    pub _control_ent: u8,

    /// Reference currently being followed.
    pub _reference: Option<Box<Reference>>,

    pub _state: u8,

    pub _proximity: u8,
}

impl FollowRefState {
    pub fn new() -> FollowRefState {
        let mut msg = FollowRefState {
            header: Header::new(480),

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _reference: Default::default(),
            _state: Default::default(),
            _proximity: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for FollowRefState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        480
    }

    fn clear(&mut self) {
        self.header.clear();

        self._control_src = Default::default();

        self._control_ent = Default::default();

        match &mut self._reference {
            Some(field) => field.clear(),

            None => {}
        }

        self._state = Default::default();

        self._proximity = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        match &self._reference {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        match &self._reference {
            None => {}

            Some(m) => m.serialize_fields(bfr),
        };
        bfr.put_u8(self._state);
        bfr.put_u8(self._proximity);
    }
}
