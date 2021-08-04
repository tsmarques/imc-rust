use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Crude oil measurement.
#[derive(Default)]
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
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }
}
