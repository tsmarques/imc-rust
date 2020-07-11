use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 5;

pub enum operationEnum {
    // Report
    OP_REPORT = 0,
    // Query
    OP_QUERY = 1,
}

impl operationEnum {
    pub fn as_u32(&self) -> u32 {
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._list = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._list.as_bytes());

        serialize_footer(bfr);
    }
}
