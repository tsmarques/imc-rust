use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

/// Report of salinity.
pub struct Salinity {
    /// IMC Header
    pub header: Header,

    /// The value of the salinity as measured by the sensor.
    pub _value: f32,
}

impl Salinity {
    pub fn new() -> Salinity {
        let mut msg = Salinity {
            header: Header::new(270),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Salinity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        270
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._value);

        serialize_footer(bfr);
    }
}
