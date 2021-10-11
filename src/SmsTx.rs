use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Request to send SMS.
#[derive(Default)]
pub struct SmsTx {
    /// IMC Header
    pub header: Header,

    /// Sequence number.
    pub _seq: u32,

    /// Number or name of the recipient.
    pub _destination: String,

    /// Timeout for sending message.
    pub _timeout: u16,

    /// Message data.
    pub _data: Vec<u8>,
}

impl Message for SmsTx {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = SmsTx {
            header: Header::new(157),

            _seq: Default::default(),
            _destination: Default::default(),
            _timeout: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = SmsTx {
            header: hdr,

            _seq: Default::default(),
            _destination: Default::default(),
            _timeout: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        157
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        157
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();
        self._destination = Default::default();
        self._timeout = Default::default();
        self._data = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._destination.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._seq);
        serialize_bytes!(bfr, self._destination.as_bytes());
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u32_le();
        deserialize_string!(bfr, self._destination);
        self._timeout = bfr.get_u16_le();
        deserialize_bytes!(bfr, self._data);

        Ok(())
    }
}
