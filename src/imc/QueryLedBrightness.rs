use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Query the brightness of an LED (Light-Emitting Diode). The
/// recipient of this message shall reply with 'LedBrightness'.
pub struct QueryLedBrightness {
    /// IMC Header
    pub header: Header,

    /// LED name.
    pub _name: String,
}

impl QueryLedBrightness {
    pub fn new() -> QueryLedBrightness {
        let mut msg = QueryLedBrightness {
            header: Header::new(313),

            _name: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for QueryLedBrightness {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        313
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());

        serialize_footer(bfr);
    }
}
