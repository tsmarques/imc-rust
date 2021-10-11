use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum StatusCodeEnum {
    // Successfull transmission
    TXSTATUS_OK = 1,
    // Error while trying to transmit message
    TXSTATUS_ERROR = 2,
    // Message has been queued for transmission
    TXSTATUS_QUEUED = 3,
    // Message is currently being transmitted
    TXSTATUS_TRANSMIT = 4,
    // Message's TTL has expired. Transmition cancelled.
    TXSTATUS_EXPIRED = 5,
}

impl StatusCodeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            TXSTATUS_OK => 1,
            TXSTATUS_ERROR => 2,
            TXSTATUS_QUEUED => 3,
            TXSTATUS_TRANSMIT => 4,
            TXSTATUS_EXPIRED => 5,
        }
    }
}

#[derive(Default)]
pub struct IridiumTxStatus {
    /// IMC Header
    pub header: Header,

    /// The request identifier used to receive transmission updates
    pub _req_id: u16,

    pub _status: u8,

    pub _text: String,
}

impl Message for IridiumTxStatus {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = IridiumTxStatus {
            header: Header::new(172),

            _req_id: Default::default(),
            _status: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = IridiumTxStatus {
            header: hdr,

            _req_id: Default::default(),
            _status: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        172
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        172
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._req_id = Default::default();
        self._status = Default::default();
        self._text = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._status);
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._status = bfr.get_u8();
        deserialize_string!(bfr, self._text);

        Ok(())
    }
}
