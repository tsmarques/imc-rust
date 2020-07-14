use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Report of temperature.
pub struct Temperature {
    /// IMC Header
    pub header: Header,

    /// The value of the temperature as measured by the sensor.
    pub _value: f32,
}

impl Temperature {
    pub fn new() -> Temperature {
        let mut msg = Temperature {
            header: Header::new(263),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Temperature {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        263
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
