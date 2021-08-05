use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

use crate::Command::Command;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Waiting for first command
    FC_WAIT = 1,
    // Moving towards received command
    FC_MOVING = 2,
    // Speed command is zero
    FC_STOPPED = 3,
    // Command is out of safe bounds
    FC_BAD_COMMAND = 4,
    // Controlling system timed out
    FC_TIMEOUT = 5,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            FC_WAIT => 1,
            FC_MOVING => 2,
            FC_STOPPED => 3,
            FC_BAD_COMMAND => 4,
            FC_TIMEOUT => 5,
        }
    }
}

/// Maneuver will be terminated since timeout was exceeded.
#[derive(Default)]
pub struct FollowCommandState {
    /// IMC Header
    pub header: Header,

    /// The IMC identifier of the source system that is allowed to control the vehicle.
    /// If the value ''0xFFFF'' is used, any system is allowed to command references.
    pub _control_src: u16,

    /// The entity identifier of the entity that is allowed to control the vehicle.
    /// If the value ''0xFF'' is used, any entity is allowed to command references.
    pub _control_ent: u8,

    /// Command currently being followed.
    pub _command: Option<Box<Command>>,

    /// Command provided breaks system's physical limitations.
    pub _state: u8,
}

impl FollowCommandState {
    pub fn new() -> FollowCommandState {
        let mut msg = FollowCommandState {
            header: Header::new(498),

            _control_src: Default::default(),
            _control_ent: Default::default(),
            _command: Default::default(),
            _state: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for FollowCommandState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        498
    }

    fn clear(&mut self) {
        self.header.clear();

        self._control_src = Default::default();

        self._control_ent = Default::default();

        match &mut self._command {
            Some(field) => field.clear(),

            None => {}
        }

        self._state = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        match &self._command {
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
        match &self._command {
            None => {}

            Some(m) => m.serialize_fields(bfr),
        };
        bfr.put_u8(self._state);
    }
}
