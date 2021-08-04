use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct QueryEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,

    pub _visibility: String,

    pub _scope: String,
}

impl QueryEntityParameters {
    pub fn new() -> QueryEntityParameters {
        let mut msg = QueryEntityParameters {
            header: Header::new(803),

            _name: Default::default(),
            _visibility: Default::default(),
            _scope: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for QueryEntityParameters {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        803
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._visibility = Default::default();

        self._scope = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._visibility.len() + 2;

        dyn_size += self._scope.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._visibility.as_bytes());
        serialize_bytes!(bfr, self._scope.as_bytes());
    }
}
