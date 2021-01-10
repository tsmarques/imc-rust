use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum OperationEnum {
    // Stop Braking
    OP_STOP = 0,
    // Start Braking
    OP_START = 1,
    // Revert Actuation
    OP_REVERT = 2,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            OP_STOP => 0,
            OP_START => 1,
            OP_REVERT => 2,
        }
    }
}

/// Revert Actuation.
pub struct Brake {
    /// IMC Header
    pub header: Header,

    /// Start braking procedures.
    pub _op: u8,
}

impl Brake {
    pub fn new() -> Brake {
        let mut msg = Brake {
            header: Header::new(413),

            _op: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Brake {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        413
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);

        serialize_footer(bfr);
    }
}
