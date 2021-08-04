use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Report of pH.
#[derive(Default)]
pub struct PH {
    /// IMC Header
    pub header: Header,

    /// The value of the pH as measured by the sensor.
    pub _value: f32,
}

impl PH {
    pub fn new() -> PH {
        let mut msg = PH {
            header: Header::new(298),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PH {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        298
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
