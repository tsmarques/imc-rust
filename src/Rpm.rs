use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Number of revolutions per minute.
#[derive(Default)]
pub struct Rpm {
    /// IMC Header
    pub header: Header,

    /// Number of revolutions per minute.
    pub _value: i16,
}

impl Rpm {
    pub fn new() -> Rpm {
        let mut msg = Rpm {
            header: Header::new(250),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Rpm {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        250
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_i16_le(self._value);
    }
}
