use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Measurement of relative humidity.
#[derive(Default)]
pub struct RelativeHumidity {
    /// IMC Header
    pub header: Header,

    /// Value of relative humidity.
    pub _value: f32,
}

impl RelativeHumidity {
    pub fn new() -> RelativeHumidity {
        let mut msg = RelativeHumidity {
            header: Header::new(272),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RelativeHumidity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        272
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
