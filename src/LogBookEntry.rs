use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum TypeEnum {
    // Information
    LBET_INFO = 0,
    // Warning
    LBET_WARNING = 1,
    // Error
    LBET_ERROR = 2,
    // Critical
    LBET_CRITICAL = 3,
    // Debug
    LBET_DEBUG = 4,
}

impl TypeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            LBET_INFO => 0,
            LBET_WARNING => 1,
            LBET_ERROR => 2,
            LBET_CRITICAL => 3,
            LBET_DEBUG => 4,
        }
    }
}

/// Human readable message reporting an event of interest.
#[derive(Default)]
pub struct LogBookEntry {
    /// IMC Header
    pub header: Header,

    /// Type of message.
    pub _type: u8,

    /// Timestamp (Epoch time).
    pub _htime: f64,

    /// Message context.
    pub _context: String,

    /// Message text.
    pub _text: String,
}

impl Message for LogBookEntry {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = LogBookEntry {
            header: hdr,

            _type: Default::default(),
            _htime: Default::default(),
            _context: Default::default(),
            _text: Default::default(),
        };

        msg.get_header()._mgid = 103;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = LogBookEntry {
            header: Header::new(103),

            _type: Default::default(),
            _htime: Default::default(),
            _context: Default::default(),
            _text: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        103
    }

    fn id(&self) -> u16 {
        103
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._htime = Default::default();

        self._context = Default::default();

        self._text = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._context.len() + 2;

        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_f64_le(self._htime);
        serialize_bytes!(bfr, self._context.as_bytes());
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
