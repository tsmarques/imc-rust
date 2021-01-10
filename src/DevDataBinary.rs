use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Verbatim representation of device data in binary format.
pub struct DevDataBinary {
    /// IMC Header
    pub header: Header,

    /// Raw binary data as extracted directly from the device.
    pub _value: Vec<u8>,
}

impl DevDataBinary {
    pub fn new() -> DevDataBinary {
        let mut msg = DevDataBinary {
            header: Header::new(274),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DevDataBinary {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        274
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

        serialize_bytes!(bfr, self._value.as_slice());

        serialize_footer(bfr);
    }
}
