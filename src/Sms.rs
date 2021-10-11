use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

/// Send a SMS message.
#[derive(Default)]
pub struct Sms {
    /// IMC Header
    pub header: Header,

    /// Target mobile device number.
    pub _number: String,

    /// Timeout for sending message.
    pub _timeout: u16,

    /// Message contents.
    pub _contents: String,
}

impl Message for Sms {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Sms {
            header: Header::new(156),

            _number: Default::default(),
            _timeout: Default::default(),
            _contents: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Sms {
            header: hdr,

            _number: Default::default(),
            _timeout: Default::default(),
            _contents: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        156
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        156
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._number = Default::default();

        self._timeout = Default::default();

        self._contents = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._number.len() + 2;

        dyn_size += self._contents.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._number.as_bytes());
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._contents.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._number);

        self._timeout = bfr.get_u16_le();

        deserialize_string!(bfr, self._contents);
    }
}
