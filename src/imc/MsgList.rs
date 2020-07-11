use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 20;

pub struct MsgList {
    /// IMC Header
    pub header: Header,

    pub _msgs: Vec<Box<dyn Message>>,
}

impl MsgList {
    pub fn new() -> MsgList {
        let mut msg = MsgList {
            header: Header::new(c_msg_id),

            _msgs: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for MsgList {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        for msg in self._msgs.iter_mut() {
            msg.clear();
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        for msg in self._msgs.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
