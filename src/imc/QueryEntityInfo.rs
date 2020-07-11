use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 4;

/// Request information about an entity identifier. The receiving
/// system shall reply with an EntityInfo message with the details
/// of that entity.
pub struct QueryEntityInfo {
    /// IMC Header
    pub header: Header,

    /// Entity identifier.
    pub _id: u8,
}

impl QueryEntityInfo {
    pub fn new() -> QueryEntityInfo {
        let mut msg = QueryEntityInfo {
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._id);

        serialize_footer(bfr);
    }
}
