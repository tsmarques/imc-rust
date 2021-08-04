#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Set the state of a given GPIO. The receiving entity shall reply
/// with a GpioState message.
#[derive(Default)]
pub struct GpioStateSet {
    /// IMC Header
    pub header: Header,

    /// GPIO Name.
    pub _name: String,

    /// Logical level of the GPIO.
    pub _value: u8,
}

impl GpioStateSet {
    pub fn new() -> GpioStateSet {
        let mut msg = GpioStateSet {
            header: Header::new(2002),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GpioStateSet {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2002
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._value);
    }
}
