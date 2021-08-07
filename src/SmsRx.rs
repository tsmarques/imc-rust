use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Received SMS data.
#[derive(Default)]
pub struct SmsRx {
    /// IMC Header
    pub header: Header,

    /// Number of name of the sender.
    pub _source: String,

    /// Message data.
    pub _data: Vec<u8>,
}

impl Message for SmsRx {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = SmsRx {
            header: hdr,

            _source: Default::default(),
            _data: Default::default(),
        };

        msg.get_header()._mgid = 158;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = SmsRx {
            header: Header::new(158),

            _source: Default::default(),
            _data: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        158
    }

    fn id(&self) -> u16 {
        158
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._source = Default::default();

        self._data = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._source.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._source.as_bytes());
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
