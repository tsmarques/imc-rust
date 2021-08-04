use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Rhodamine Dye measurement.
#[derive(Default)]
pub struct RhodamineDye {
    /// IMC Header
    pub header: Header,

    /// Amount of rhodamine dye detected.
    pub _value: f32,
}

impl RhodamineDye {
    pub fn new() -> RhodamineDye {
        let mut msg = RhodamineDye {
            header: Header::new(285),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RhodamineDye {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        285
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
