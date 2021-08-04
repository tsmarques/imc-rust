use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// This message is sent in reply to an AcousticSystemsQuery message
/// and lists all known underwater acoustic systems (modems, narrow
/// band transponders, etc).
#[derive(Default)]
pub struct AcousticSystems {
    /// IMC Header
    pub header: Header,

    /// Comma separated list of known acoustic system names.
    pub _list: String,
}

impl AcousticSystems {
    pub fn new() -> AcousticSystems {
        let mut msg = AcousticSystems {
            header: Header::new(213),

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
        213
    }

    fn clear(&mut self) {
        self.header.clear();

        self._list = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._list.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._list.as_bytes());
    }
}
