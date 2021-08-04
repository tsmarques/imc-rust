#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum TypeEnum {
    // Boolean
    PVT_BOOLEAN = 0,
    // Number
    PVT_NUMBER = 1,
    // Text
    PVT_TEXT = 2,
    // Message
    PVT_MESSAGE = 3,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            PVT_BOOLEAN => 0,
            PVT_NUMBER => 1,
            PVT_TEXT => 2,
            PVT_MESSAGE => 3,
        }
    }
}

pub enum AccessTypeEnum {
    // Input
    PVA_INPUT = 0,
    // Output
    PVA_OUTPUT = 1,
    // Local
    PVA_LOCAL = 2,
}

impl AccessTypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            PVA_INPUT => 0,
            PVA_OUTPUT => 1,
            PVA_LOCAL => 2,
        }
    }
}

/// A plan variable.
#[derive(Default)]
pub struct PlanVariable {
    /// IMC Header
    pub header: Header,

    pub _name: String,

    pub _value: String,

    pub _type: u8,

    pub _access: u8,
}

impl PlanVariable {
    pub fn new() -> PlanVariable {
        let mut msg = PlanVariable {
            header: Header::new(561),

            _name: Default::default(),
            _value: Default::default(),
            _type: Default::default(),
            _access: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanVariable {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        561
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();

        self._type = Default::default();

        self._access = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._value.as_bytes());
        bfr.put_u8(self._type);
        bfr.put_u8(self._access);
    }
}
