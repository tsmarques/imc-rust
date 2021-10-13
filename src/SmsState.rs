use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Accepted
    SMS_ACCEPTED = 0,
    // Rejected
    SMS_REJECTED = 1,
    // Interrupted
    SMS_INTERRUPTED = 2,
    // Completed
    SMS_COMPLETED = 3,
    // Idle
    SMS_IDLE = 4,
    // Transmitting
    SMS_TRANSMITTING = 5,
    // Receiving
    SMS_RECEIVING = 6,
}

#[allow(non_camel_case_types)]
pub mod State {
    // Accepted
    pub const SMS_ACCEPTED: u32 = 0;
    // Rejected
    pub const SMS_REJECTED: u32 = 1;
    // Interrupted
    pub const SMS_INTERRUPTED: u32 = 2;
    // Completed
    pub const SMS_COMPLETED: u32 = 3;
    // Idle
    pub const SMS_IDLE: u32 = 4;
    // Transmitting
    pub const SMS_TRANSMITTING: u32 = 5;
    // Receiving
    pub const SMS_RECEIVING: u32 = 6;
}

#[derive(Default)]
pub struct SmsState {
    /// IMC Header
    pub header: Header,

    /// Sequence number.
    pub _seq: u32,

    /// Current state of an SMS transaction.
    pub _state: u8,

    pub _error: String,
}

impl Message for SmsState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = SmsState {
            header: Header::new(159),

            _seq: Default::default(),
            _state: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = SmsState {
            header: hdr,

            _seq: Default::default(),
            _state: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        159
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        159
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();
        self._state = Default::default();
        self._error = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._seq);
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u32_le();
        self._state = bfr.get_u8();
        deserialize_string!(bfr, self._error);

        Ok(())
    }
}
