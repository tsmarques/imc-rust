use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 813;

pub enum TypeEnum {
    // Input Available
    IOV_TYPE_INPUT = 1,
    // Input Error
    IOV_TYPE_INPUT_ERROR = 2,
}

impl TypeEnum {
    pub fn as_u32(&self) -> u32 {
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._error = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._type);
        serialize_bytes!(bfr, self._error.as_bytes());

        serialize_footer(bfr);
    }
}
