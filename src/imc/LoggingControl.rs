use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

// This operation instructs the logging manager to send a
// message with operation CURRENT_NAME containing the complete
// name of the current log in the field 'name'. The field
// 'name' with this operation type has no meaning.

const c_msg_id: u16 = 102;

pub enum op {
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

impl op {
    pub fn as_u8(&self) -> u8 {
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

pub struct LoggingControl {
    pub header: Header,
    // The logging manager will send a message with this operation
    // when asked via the REQUEST_CURRENT_NAME operation. The field
    // 'name' contains the complete name of the log.
    pub op: u8,

    // The meaning of this field depends on the operation and is
    // explained in the operation's description.
    pub name: String,
}

impl LoggingControl {
    pub fn new() -> LoggingControl {
        let mut msg = LoggingControl {
            header: Header::new(c_msg_id),
            op: 0,
            name: String::new(),
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
        c_msg_id
    }

    fn clear(&mut self) {}

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);
        bfr.put_u8(self.op);
        serialize_string!(bfr, self.name);
        serialize_footer(bfr);
    }
}
