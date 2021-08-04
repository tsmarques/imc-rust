#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum TypeEnum {
    // X-axis
    CD_X = 0x01,
    // Y-axis
    CD_Y = 0x02,
    // Z-axis
    CD_Z = 0x04,
    // Impact
    CD_IMPACT = 0x08,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            CD_X => 0x01,
            CD_Y => 0x02,
            CD_Z => 0x04,
            CD_IMPACT => 0x08,
        }
    }
}

/// Collision detected in the z-axis
#[derive(Default)]
pub struct Collision {
    /// IMC Header
    pub header: Header,

    /// Estimated collision acceleration value.
    pub _value: f32,

    /// Sudden impact detected
    pub _type: u8,
}

impl Collision {
    pub fn new() -> Collision {
        let mut msg = Collision {
            header: Header::new(509),

            _value: Default::default(),
            _type: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Collision {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        509
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();

        self._type = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._type);
    }
}
