use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum AttributetypeEnum {
    // Boolean Domain
    TYPE_BOOL = 1,
    // Integer Domain
    TYPE_INT = 2,
    // Float Domain
    TYPE_FLOAT = 3,
    // String Domain
    TYPE_STRING = 4,
    // Enumerated Domain
    TYPE_ENUM = 5,
}

impl AttributetypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            TYPE_BOOL => 1,
            TYPE_INT => 2,
            TYPE_FLOAT => 3,
            TYPE_STRING => 4,
            TYPE_ENUM => 5,
        }
    }
}

pub struct TrexAttribute {
    /// IMC Header
    pub header: Header,

    /// Name of this attribute.
    pub _name: String,

    pub _attr_type: u8,

    /// Lower bound of this interval. Empty text means no bound.
    pub _min: String,

    /// Upper bound of this interval. Empty text means no bound.
    pub _max: String,
}

impl TrexAttribute {
    pub fn new() -> TrexAttribute {
        let mut msg = TrexAttribute {
            header: Header::new(656),

            _name: Default::default(),
            _attr_type: Default::default(),
            _min: Default::default(),
            _max: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TrexAttribute {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        656
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._attr_type = Default::default();

        self._min = Default::default();

        self._max = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._min.len() + 2;

        dyn_size += self._max.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._attr_type);
        serialize_bytes!(bfr, self._min.as_bytes());
        serialize_bytes!(bfr, self._max.as_bytes());

        serialize_footer(bfr);
    }
}
