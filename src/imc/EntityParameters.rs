use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

use crate::imc::EntityParameter::EntityParameter;

/// List of entity parameters.
pub struct EntityParameters {
    /// IMC Header
    pub header: Header,

    /// Name of the entity.
    pub _name: String,

    /// List of parameters.
    pub _params: Vec<Box<EntityParameter>>,
}

impl EntityParameters {
    pub fn new() -> EntityParameters {
        let mut msg = EntityParameters {
            header: Header::new(802),

            _name: Default::default(),
            _params: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EntityParameters {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        802
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        for msg in self._params.iter_mut() {
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

        serialize_bytes!(bfr, self._name.as_bytes());
        for msg in self._params.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
