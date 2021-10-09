use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::TrexAttribute::TrexAttribute;

#[derive(Default)]
pub struct TrexToken {
    /// IMC Header
    pub header: Header,

    pub _timeline: String,

    pub _predicate: String,

    pub _attributes: MessageList<TrexAttribute>,
}

impl Message for TrexToken {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = TrexToken {
            header: Header::new(657),

            _timeline: Default::default(),
            _predicate: Default::default(),
            _attributes: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = TrexToken {
            header: hdr,

            _timeline: Default::default(),
            _predicate: Default::default(),
            _attributes: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        657
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        657
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._timeline = Default::default();

        self._predicate = Default::default();

        self._attributes = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._timeline.len() + 2;

        dyn_size += self._predicate.len() + 2;

        message_list_serialization_size!(dyn_size, self._attributes);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._timeline.as_bytes());
        serialize_bytes!(bfr, self._predicate.as_bytes());
        serialize_message_list!(bfr, self._attributes);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._timeline);

        deserialize_string!(bfr, self._predicate);

        for m in self._attributes.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
