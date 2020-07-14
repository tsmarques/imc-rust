use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Request a list of known underwater acoustic systems. The
/// recipient of this message shall reply with an AcousticSystems
/// message.
pub struct AcousticSystemsQuery {
    /// IMC Header
    pub header: Header,
}

impl AcousticSystemsQuery {
    pub fn new() -> AcousticSystemsQuery {
        let mut msg = AcousticSystemsQuery {
            header: Header::new(212),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for AcousticSystemsQuery {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        212
    }

    fn clear(&mut self) {
        self.header.clear();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_footer(bfr);
    }
}
