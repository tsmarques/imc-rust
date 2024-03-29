use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

use crate::imc::LogBookEntry::LogBookEntry;

pub enum CommandEnum {
    // Get
    LBC_GET = 0,
    // Clear
    LBC_CLEAR = 1,
    // Get Errors
    LBC_GET_ERR = 2,
    // Reply
    LBC_REPLY = 3,
}

impl CommandEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            LBC_GET => 0,
            LBC_CLEAR => 1,
            LBC_GET_ERR => 2,
            LBC_REPLY => 3,
        }
    }
}

/// Retrieve log book entries corresponding to errors.
pub struct LogBookControl {
    /// IMC Header
    pub header: Header,

    /// Reply to a GET command. Message argument is a MessageList
    /// instance containing LogBookEntry messages.
    pub _command: u8,

    /// Timestamp for command (Epoch time).
    pub _htime: f64,

    /// Argument, currently used only for 'REPLY'.
    pub _msg: Vec<Box<LogBookEntry>>,
}

impl LogBookControl {
    pub fn new() -> LogBookControl {
        let mut msg = LogBookControl {
            header: Header::new(104),

            _command: Default::default(),
            _htime: Default::default(),
            _msg: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LogBookControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        104
    }

    fn clear(&mut self) {
        self.header.clear();

        self._command = Default::default();

        self._htime = Default::default();

        for msg in self._msg.iter_mut() {
            msg.clear();
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        for msg in &self._msg {
            dyn_size += msg.dynamic_serialization_size();
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._command);
        bfr.put_f64_le(self._htime);
        for msg in self._msg.iter() {
            msg.serialize(bfr);
        }

        serialize_footer(bfr);
    }
}
