use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Request the state of a given GPIO. The receiving entity shall reply
/// with a GpioState message.
pub struct GpioStateGet {
    /// IMC Header
    pub header: Header,

    /// GPIO Name.
    pub _name: String,
}

impl GpioStateGet {
    pub fn new() -> GpioStateGet {
        let mut msg = GpioStateGet {
            header: Header::new(2001),

            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GpioStateGet {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2001
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());

        serialize_footer(bfr);
    }
}
