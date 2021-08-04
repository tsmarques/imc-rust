use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

use crate::EntityParameter::EntityParameter;

#[derive(Default)]
pub struct SetEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,

    pub _params: MessageList<EntityParameter>,
}

impl SetEntityParameters {
    pub fn new() -> SetEntityParameters {
        let mut msg = SetEntityParameters {
            header: Header::new(804),

            _name: Default::default(),
            _params: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SetEntityParameters {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        804
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        for msg in self._params.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.clear();
                }
            }
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        for msg in self._params.iter() {
            match msg {
                None => {}
                Some(m) => {
                    dyn_size += m.dynamic_serialization_size();
                }
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        for msg in self._params.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
