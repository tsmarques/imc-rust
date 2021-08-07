use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Stop Braking
    OP_STOP = 0,
    // Start Braking
    OP_START = 1,
    // Revert Actuation
    OP_REVERT = 2,
}

impl OperationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            OP_STOP => 0,
            OP_START => 1,
            OP_REVERT => 2,
        }
    }
}

/// Revert Actuation.
#[derive(Default)]
pub struct Brake {
    /// IMC Header
    pub header: Header,

    /// Start braking procedures.
    pub _op: u8,
}

impl Message for Brake {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = Brake {
            header: hdr,

            _op: Default::default(),
        };

        msg.get_header()._mgid = 413;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = Brake {
            header: Header::new(413),

            _op: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        413
    }

    fn id(&self) -> u16 {
        413
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
