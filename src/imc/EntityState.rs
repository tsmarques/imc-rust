use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 1;

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
    pub fn as_u32(&self) -> u32 {
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
    pub fn as_u32(&self) -> u32 {
        match self {
            EFLA_HUMAN_INTERVENTION => 0x01,
        }
    }
}

/// State reported by an entity in the vehicle. The source entity is
/// identified in the message header.
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();

        self._flags = Default::default();

        self._description = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._state);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._description.as_bytes());

        serialize_footer(bfr);
    }
}
