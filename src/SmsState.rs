#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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

impl StateEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            SMS_ACCEPTED => 0,
            SMS_REJECTED => 1,
            SMS_INTERRUPTED => 2,
            SMS_COMPLETED => 3,
            SMS_IDLE => 4,
            SMS_TRANSMITTING => 5,
            SMS_RECEIVING => 6,
        }
    }
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

impl SmsState {
    pub fn new() -> SmsState {
        let mut msg = SmsState {
            header: Header::new(159),

            _seq: Default::default(),
            _state: Default::default(),
            _error: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SmsState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        159
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._state = Default::default();

        self._error = Default::default();
    }

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
}
