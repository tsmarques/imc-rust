#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Request to send SMS.
#[derive(Default)]
pub struct SmsTx {
    /// IMC Header
    pub header: Header,

    /// Sequence number.
    pub _seq: u32,

    /// Number or name of the recipient.
    pub _destination: String,

    /// Timeout for sending message.
    pub _timeout: u16,

    /// Message data.
    pub _data: Vec<u8>,
}

impl SmsTx {
    pub fn new() -> SmsTx {
        let mut msg = SmsTx {
            header: Header::new(157),

            _seq: Default::default(),
            _destination: Default::default(),
            _timeout: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SmsTx {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        157
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._destination = Default::default();

        self._timeout = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._destination.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._seq);
        serialize_bytes!(bfr, self._destination.as_bytes());
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._data.as_slice());
    }
}
