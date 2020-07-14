use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Dissolved Oxygen measurement.
pub struct DissolvedOxygen {
    /// IMC Header
    pub header: Header,

    /// Dissolved Oxygen reading.
    pub _value: f32,
}

impl DissolvedOxygen {
    pub fn new() -> DissolvedOxygen {
        let mut msg = DissolvedOxygen {
            header: Header::new(295),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DissolvedOxygen {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        295
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
