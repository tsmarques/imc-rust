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

impl Message for MsgList {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = MsgList {
            header: Header::new(20),

            _msgs: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = MsgList {
            header: hdr,

            _msgs: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        20
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        20
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        for msg in self._msgs.iter_mut() {
            match msg {
                None => {}

                Some(m) => {
                    m.deserialize_fields(bfr);
                }
            }
        }
    }
}
