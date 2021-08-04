#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Message generated when tasks bind to messages.
#[derive(Default)]
pub struct TransportBindings {
    /// IMC Header
    pub header: Header,

    /// The name of the consumer (e.g. task name).
    pub _consumer: String,

    /// The id of the message to be listened to.
    pub _message_id: u16,
}

impl TransportBindings {
    pub fn new() -> TransportBindings {
        let mut msg = TransportBindings {
            header: Header::new(8),

            _consumer: Default::default(),
            _message_id: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TransportBindings {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        8
    }

    fn clear(&mut self) {
        self.header.clear();

        self._consumer = Default::default();

        self._message_id = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._consumer.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._consumer.as_bytes());
        bfr.put_u16_le(self._message_id);
    }
}
