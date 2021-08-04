#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Request information about an entity identifier. The receiving
/// system shall reply with an EntityInfo message with the details
/// of that entity.
#[derive(Default)]
pub struct QueryEntityInfo {
    /// IMC Header
    pub header: Header,

    /// Entity identifier.
    pub _id: u8,
}

impl QueryEntityInfo {
    pub fn new() -> QueryEntityInfo {
        let mut msg = QueryEntityInfo {
            header: Header::new(4),

            _id: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for QueryEntityInfo {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        4
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
    }
}
