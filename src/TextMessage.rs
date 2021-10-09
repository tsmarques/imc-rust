use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// A text message has been received.
#[derive(Default)]
pub struct TextMessage {
    /// IMC Header
    pub header: Header,

    /// Message origin (if known).
    pub _origin: String,

    /// Message contents.
    pub _text: String,
}

impl Message for TextMessage {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = TextMessage {
            header: Header::new(160),

            _origin: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = TextMessage {
            header: hdr,

            _origin: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        160
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        160
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._origin = Default::default();

        self._text = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._origin.len() + 2;

        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._origin.as_bytes());
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._origin);

        deserialize_string!(bfr, self._text);
    }
}
