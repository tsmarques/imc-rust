use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::TrexAttribute::TrexAttribute;

pub struct TrexToken {
    /// IMC Header
    pub header: Header,

    pub _timeline: String,

    pub _predicate: String,

    pub _attributes: Vec<Box<TrexAttribute>>,
}

impl TrexToken {
    pub fn new() -> TrexToken {
        let mut msg = TrexToken {
            header: Header::new(657),

            _timeline: Default::default(),
            _predicate: Default::default(),
            _attributes: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TrexToken {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        657
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeline = Default::default();

        self._predicate = Default::default();

        for msg in self._attributes.iter_mut() {
            msg.clear();
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._timeline.len() + 2;

        dyn_size += self._predicate.len() + 2;

        for msg in &self._attributes {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._timeline.as_bytes());
        serialize_bytes!(bfr, self._predicate.as_bytes());
        for msg in self._attributes.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
