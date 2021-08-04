#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum ControlOperationEnum {
    // Request Start of Logging
    COP_REQUEST_START = 0,
    // Logging Started
    COP_STARTED = 1,
    // Request Logging Stop
    COP_REQUEST_STOP = 2,
    // Logging Stopped
    COP_STOPPED = 3,
    // Request Current Log Name
    COP_REQUEST_CURRENT_NAME = 4,
    // Current Log Name
    COP_CURRENT_NAME = 5,
}

impl ControlOperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            COP_REQUEST_START => 0,
            COP_STARTED => 1,
            COP_REQUEST_STOP => 2,
            COP_STOPPED => 3,
            COP_REQUEST_CURRENT_NAME => 4,
            COP_CURRENT_NAME => 5,
        }
    }
}

/// This operation instructs the logging manager to send a
/// message with operation CURRENT_NAME containing the complete
/// name of the current log in the field 'name'. The field
/// 'name' with this operation type has no meaning.
#[derive(Default)]
pub struct LoggingControl {
    /// IMC Header
    pub header: Header,

    /// The logging manager will send a message with this operation
    /// when asked via the REQUEST_CURRENT_NAME operation. The field
    /// 'name' contains the complete name of the log.
    pub _op: u8,

    /// The meaning of this field depends on the operation and is
    /// explained in the operation's description.
    pub _name: String,
}

impl LoggingControl {
    pub fn new() -> LoggingControl {
        let mut msg = LoggingControl {
            header: Header::new(102),

            _op: Default::default(),
            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LoggingControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        102
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._name.as_bytes());
    }
}
