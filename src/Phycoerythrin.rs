use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Phycoerythrin measurement.
#[derive(Default)]
pub struct Phycoerythrin {
    /// IMC Header
    pub header: Header,

    /// Phycoerythrin reading.
    pub _value: f32,
}

impl Phycoerythrin {
    pub fn new() -> Phycoerythrin {
        let mut msg = Phycoerythrin {
            header: Header::new(292),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Phycoerythrin {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        292
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
