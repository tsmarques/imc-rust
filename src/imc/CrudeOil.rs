use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Crude oil measurement.
pub struct CrudeOil {
    /// IMC Header
    pub header: Header,

    /// Amount of crude oil detected.
    pub _value: f32,
}

impl CrudeOil {
    pub fn new() -> CrudeOil {
        let mut msg = CrudeOil {
            header: Header::new(286),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for CrudeOil {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        286
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._value);

        serialize_footer(bfr);
    }
}
