use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct SaveEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,
}

impl SaveEntityParameters {
    pub fn new() -> SaveEntityParameters {
        let mut msg = SaveEntityParameters {
            header: Header::new(805),

            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SaveEntityParameters {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        805
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
    }
}
