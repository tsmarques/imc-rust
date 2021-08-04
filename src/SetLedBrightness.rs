use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Control the brightness of an LED (Light-Emitting Diode). The
/// recipient of this message shall set the intensity of the LED to
/// the desired 'value' and reply with 'LedBrightness'.
#[derive(Default)]
pub struct SetLedBrightness {
    /// IMC Header
    pub header: Header,

    /// LED name.
    pub _name: String,

    /// Desired brightness value.
    pub _value: u8,
}

impl SetLedBrightness {
    pub fn new() -> SetLedBrightness {
        let mut msg = SetLedBrightness {
            header: Header::new(314),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for SetLedBrightness {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        314
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._value);
    }
}
