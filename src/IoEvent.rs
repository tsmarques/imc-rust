use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum TypeEnum {
    // Input Available
    IOV_TYPE_INPUT = 1,
    // Input Error
    IOV_TYPE_INPUT_ERROR = 2,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            IOV_TYPE_INPUT => 1,
            IOV_TYPE_INPUT_ERROR => 2,
        }
    }
}

/// Notification of an I/O event.
pub struct IoEvent {
    /// IMC Header
    pub header: Header,

    /// Event type.
    pub _type: u8,

    /// Human-readable error message.
    pub _error: String,
}

impl IoEvent {
    pub fn new() -> IoEvent {
        let mut msg = IoEvent {
            header: Header::new(813),

            _type: Default::default(),
            _error: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for IoEvent {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        813
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._error = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._type);
        serialize_bytes!(bfr, self._error.as_bytes());

        serialize_footer(bfr);
    }
}
