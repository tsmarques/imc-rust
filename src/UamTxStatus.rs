use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum ValueEnum {
    // Transmission Completed
    UTS_DONE = 0,
    // Transmission Failed
    UTS_FAILED = 1,
    // Transmission Canceled
    UTS_CANCELED = 2,
    // Modem is busy
    UTS_BUSY = 3,
    // Invalid address
    UTS_INV_ADDR = 4,
    // In Progress
    UTS_IP = 5,
    // Unsupported operation
    UTS_UNSUPPORTED = 6,
    // Invalid transmission size
    UTS_INV_SIZE = 7,
    // Not transducer
    UTS_NOT_TRANSDUCER = 8,
}

#[allow(non_camel_case_types)]
pub mod Value {
    // Transmission Completed
    pub const UTS_DONE: u32 = 0;
    // Transmission Failed
    pub const UTS_FAILED: u32 = 1;
    // Transmission Canceled
    pub const UTS_CANCELED: u32 = 2;
    // Modem is busy
    pub const UTS_BUSY: u32 = 3;
    // Invalid address
    pub const UTS_INV_ADDR: u32 = 4;
    // In Progress
    pub const UTS_IP: u32 = 5;
    // Unsupported operation
    pub const UTS_UNSUPPORTED: u32 = 6;
    // Invalid transmission size
    pub const UTS_INV_SIZE: u32 = 7;
    // Not transducer
    pub const UTS_NOT_TRANSDUCER: u32 = 8;
}

/// transducer not connected to the acoustic modem.
#[derive(Default)]
pub struct UamTxStatus {
    /// IMC Header
    pub header: Header,

    /// The sequence identifier of the frame transmission request.
    pub _seq: u16,

    /// The frame transmission request exceeds the MTU of the acoustic
    /// modem.
    pub _value: u8,

    /// Where applicable this field shall contain a human-readable message
    /// explaining the error.
    pub _error: String,
}

impl Message for UamTxStatus {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UamTxStatus {
            header: Header::new(816),

            _seq: Default::default(),
            _value: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UamTxStatus {
            header: hdr,

            _seq: Default::default(),
            _value: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        816
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        816
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();
        self._value = Default::default();
        self._error = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        bfr.put_u8(self._value);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u16_le();
        self._value = bfr.get_u8();
        deserialize_string!(bfr, self._error);

        Ok(())
    }
}
