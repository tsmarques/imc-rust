use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub struct MsgList {
    /// IMC Header
    pub header: Header,

    pub _msgs: Vec<Box<dyn Message>>,
}

impl MsgList {
    pub fn new() -> MsgList {
        let mut msg = MsgList {
            header: Header::new(20),

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
        20
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
        let mut dyn_size: usize = 0;

        for msg in &self._msgs {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        for msg in self._msgs.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
