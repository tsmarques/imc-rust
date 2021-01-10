use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Verbatim representation of device data in plain text format.
pub struct DevDataText {
    /// IMC Header
    pub header: Header,

    /// Plain text data as extracted directly from the device.
    pub _value: String,
}

impl DevDataText {
    pub fn new() -> DevDataText {
        let mut msg = DevDataText {
            header: Header::new(273),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DevDataText {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        273
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._value.as_bytes());

        serialize_footer(bfr);
    }
}
