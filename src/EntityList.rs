use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum operationEnum {
    // Report
    OP_REPORT = 0,
    // Query
    OP_QUERY = 1,
}

impl operationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            OP_REPORT => 0,
            OP_QUERY => 1,
        }
    }
}

/// This message describes the names and identification numbers of
/// all entities in the system.
pub struct EntityList {
    /// IMC Header
    pub header: Header,

    /// Operation to perform.
    pub _op: u8,

    /// Example: &quot;Battery=11;CTD=3&quot;
    pub _list: String,
}

impl EntityList {
    pub fn new() -> EntityList {
        let mut msg = EntityList {
            header: Header::new(5),

            _op: Default::default(),
            _list: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EntityList {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        5
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._list = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._list.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._list.as_bytes());

        serialize_footer(bfr);
    }
}
