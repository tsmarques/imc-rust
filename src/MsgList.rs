use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

#[derive(Default)]
pub struct MsgList {
    /// IMC Header
    pub header: Header,

    pub _msgs: MessageList<Box<dyn Message>>,
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

        self._msgs = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        message_list_serialization_size!(dyn_size, self._msgs);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_message_list!(bfr, self._msgs);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        for m in self._msgs.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
