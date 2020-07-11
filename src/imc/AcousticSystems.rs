use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 213;

/// This message is sent in reply to an AcousticSystemsQuery message
/// and lists all known underwater acoustic systems (modems, narrow
/// band transponders, etc).
pub struct AcousticSystems {
    /// IMC Header
    pub header: Header,

    /// Comma separated list of known acoustic system names.
    pub _list: String,
}

impl AcousticSystems {
    pub fn new() -> AcousticSystems {
        let mut msg = AcousticSystems {
            header: Header::new(c_msg_id),

            _list: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for AcousticSystems {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

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

        serialize_bytes!(bfr, self._list.as_bytes());

        serialize_footer(bfr);
    }
}
