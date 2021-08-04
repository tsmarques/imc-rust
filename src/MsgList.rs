use crate::Message::*;

use crate::MessageList;

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct MsgList {
    /// IMC Header
    pub header: Header,

    pub _msgs: MessageList<dyn Message>,
}

impl MsgList {
    pub fn new() -> MsgList {
        let mut msg = MsgList {
            header: Header::new(20),

            _msgs: vec![],
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for MsgList {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        20
    }

    fn clear(&mut self) {
        self.header.clear();

        for msg in self._msgs.iter_mut() {
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

        for msg in self._msgs.iter() {
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
        for msg in self._msgs.iter() {
            match msg {
                None => {}

                Some(m) => {
                    m.serialize_fields(bfr);
                }
            }
        }
    }
}
