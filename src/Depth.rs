use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Depth report.
#[derive(Default)]
pub struct Depth {
    /// IMC Header
    pub header: Header,

    /// Depth value measured by a sensor.
    pub _value: f32,
}

impl Depth {
    pub fn new() -> Depth {
        let mut msg = Depth {
            header: Header::new(265),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Depth {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        265
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
