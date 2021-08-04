use crate::Message::*;

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

impl TextMessage {
    pub fn new() -> TextMessage {
        let mut msg = TextMessage {
            header: Header::new(160),

            _origin: Default::default(),
            _text: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TextMessage {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        160
    }

    fn clear(&mut self) {
        self.header.clear();

        self._origin = Default::default();

        self._text = Default::default();
    }

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
}
