#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum StateEnum {
    // Bootstrapping
    ESTA_BOOT = 0,
    // Normal Operation
    ESTA_NORMAL = 1,
    // Fault
    ESTA_FAULT = 2,
    // Error
    ESTA_ERROR = 3,
    // Failure
    ESTA_FAILURE = 4,
}

impl StateEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            ESTA_BOOT => 0,
            ESTA_NORMAL => 1,
            ESTA_FAULT => 2,
            ESTA_ERROR => 3,
            ESTA_FAILURE => 4,
        }
    }
}

pub enum FlagsEnum {
    // Human Intervention Required
    EFLA_HUMAN_INTERVENTION = 0x01,
}

impl FlagsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            EFLA_HUMAN_INTERVENTION => 0x01,
        }
    }
}

/// State reported by an entity in the vehicle. The source entity is
/// identified in the message header.
#[derive(Default)]
pub struct EntityState {
    /// IMC Header
    pub header: Header,

    /// State of entity.
    pub _state: u8,

    /// Complementary entity state flags.
    pub _flags: u8,

    /// Complementary human-readable description of entity state.
    pub _description: String,
}

impl EntityState {
    pub fn new() -> EntityState {
        let mut msg = EntityState {
            header: Header::new(1),

            _state: Default::default(),
            _flags: Default::default(),
            _description: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EntityState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        1
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();

        self._flags = Default::default();

        self._description = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._description.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._description.as_bytes());
    }
}
