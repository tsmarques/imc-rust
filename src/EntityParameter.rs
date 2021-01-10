use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Entity parameter.
pub struct EntityParameter {
    /// IMC Header
    pub header: Header,

    /// Name of the parameter.
    pub _name: String,

    /// Current value of the parameter.
    pub _value: String,
}

impl EntityParameter {
    pub fn new() -> EntityParameter {
        let mut msg = EntityParameter {
            header: Header::new(801),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EntityParameter {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        801
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._value.as_bytes());

        serialize_footer(bfr);
    }
}
