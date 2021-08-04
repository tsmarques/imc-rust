use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

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
    pub fn as_primitive(&self) -> u32 {
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

impl IridiumTxStatus {
    pub fn new() -> IridiumTxStatus {
        let mut msg = IridiumTxStatus {
            header: Header::new(172),

            _req_id: Default::default(),
            _status: Default::default(),
            _text: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for IridiumTxStatus {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        172
    }

    fn clear(&mut self) {
        self.header.clear();

        self._req_id = Default::default();

        self._status = Default::default();

        self._text = Default::default();
    }

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
}
