use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Abort
    AOP_ABORT = 0,
    // Abort in Progress
    AOP_ABORT_IP = 1,
    // Abort Timeout
    AOP_ABORT_TIMEOUT = 2,
    // Abort Acknowledged
    AOP_ABORT_ACKED = 3,
    // Range Request
    AOP_RANGE = 4,
    // Range in Progress
    AOP_RANGE_IP = 5,
    // Range Timeout
    AOP_RANGE_TIMEOUT = 6,
    // Range Received
    AOP_RANGE_RECVED = 7,
    // Modem is Busy
    AOP_BUSY = 8,
    // Unsupported operation
    AOP_UNSUPPORTED = 9,
    // Transducer Not Detected
    AOP_NO_TXD = 10,
    // Send Message
    AOP_MSG = 11,
    // Message Send -- Queued
    AOP_MSG_QUEUED = 12,
    // Message Send -- In progress
    AOP_MSG_IP = 13,
    // Message Send -- Done
    AOP_MSG_DONE = 14,
    // Message Send -- Failure
    AOP_MSG_FAILURE = 15,
    // Send Short Message
    AOP_MSG_SHORT = 16,
}

impl OperationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            AOP_ABORT => 0,
            AOP_ABORT_IP => 1,
            AOP_ABORT_TIMEOUT => 2,
            AOP_ABORT_ACKED => 3,
            AOP_RANGE => 4,
            AOP_RANGE_IP => 5,
            AOP_RANGE_TIMEOUT => 6,
            AOP_RANGE_RECVED => 7,
            AOP_BUSY => 8,
            AOP_UNSUPPORTED => 9,
            AOP_NO_TXD => 10,
            AOP_MSG => 11,
            AOP_MSG_QUEUED => 12,
            AOP_MSG_IP => 13,
            AOP_MSG_DONE => 14,
            AOP_MSG_FAILURE => 15,
            AOP_MSG_SHORT => 16,
        }
    }
}

/// Request message over acoustic channel. The message to send
/// is specified by the 'msg' field.
#[derive(Default)]
pub struct AcousticOperation {
    /// IMC Header
    pub header: Header,

    /// Message send request could not be fulfilled.
    pub _op: u8,

    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _system: String,

    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _range: f32,

    /// Argument for message send ('MSG') requests.
    pub _msg: Option<Box<dyn Message>>,
}

impl Message for AcousticOperation {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AcousticOperation {
            header: Header::new(211),

            _op: Default::default(),
            _system: Default::default(),
            _range: Default::default(),
            _msg: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AcousticOperation {
            header: hdr,

            _op: Default::default(),
            _system: Default::default(),
            _range: Default::default(),
            _msg: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        211
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        211
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
        self._system = Default::default();
        self._range = Default::default();
        self._msg = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._system.len() + 2;

        inline_message_serialization_size!(dyn_size, self._msg);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._system.as_bytes());
        bfr.put_f32_le(self._range);
        serialize_inline_message!(bfr, self._msg);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._system);
        self._range = bfr.get_f32_le();
        self._msg = deserialize_inline(bfr).ok();

        Ok(())
    }
}
