#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct PushEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,
}

impl PushEntityParameters {
    pub fn new() -> PushEntityParameters {
        let mut msg = PushEntityParameters {
            header: Header::new(811),

            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PushEntityParameters {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        811
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
    }
}
