#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Actual position of a servo.
#[derive(Default)]
pub struct ServoPosition {
    /// IMC Header
    pub header: Header,

    /// Servo identifier.
    pub _id: u8,

    /// Value of the servo position.
    pub _value: f32,
}

impl ServoPosition {
    pub fn new() -> ServoPosition {
        let mut msg = ServoPosition {
            header: Header::new(281),

            _id: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ServoPosition {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        281
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._value);
    }
}
