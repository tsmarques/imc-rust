use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

/// Brightness value of an LED (Light-Emitting Diode).
pub struct LedBrightness {
    /// IMC Header
    pub header: Header,

    /// LED name.
    pub _name: String,

    /// Brightness value.
    pub _value: u8,
}

impl LedBrightness {
    pub fn new() -> LedBrightness {
        let mut msg = LedBrightness {
            header: Header::new(312),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LedBrightness {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        312
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._value);

        serialize_footer(bfr);
    }
}
