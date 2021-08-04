#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Received SMS data.
#[derive(Default)]
pub struct SmsRx {
    /// IMC Header
    pub header: Header,

    /// Number of name of the sender.
    pub _source: String,

    /// Message data.
    pub _data: Vec<u8>,
}

impl SmsRx {
    pub fn new() -> SmsRx {
        let mut msg = SmsRx {
            header: Header::new(158),

            _source: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SmsRx {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        158
    }

    fn clear(&mut self) {
        self.header.clear();

        self._source = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._source.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._source.as_bytes());
        serialize_bytes!(bfr, self._data.as_slice());
    }
}
