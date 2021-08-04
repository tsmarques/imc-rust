use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Desired Heading Rate speed reference value for the control layer.
#[derive(Default)]
pub struct DesiredHeadingRate {
    /// IMC Header
    pub header: Header,

    /// The value of the desired heading rate speed in radians per
    /// second.
    pub _value: f64,
}

impl DesiredHeadingRate {
    pub fn new() -> DesiredHeadingRate {
        let mut msg = DesiredHeadingRate {
            header: Header::new(408),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DesiredHeadingRate {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        408
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
    }
}
