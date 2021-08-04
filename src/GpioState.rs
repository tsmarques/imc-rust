use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Current state of a GPIO.
#[derive(Default)]
pub struct GpioState {
    /// IMC Header
    pub header: Header,

    /// GPIO Name.
    pub _name: String,

    /// Logical level of the GPIO.
    pub _value: u8,
}

impl GpioState {
    pub fn new() -> GpioState {
        let mut msg = GpioState {
            header: Header::new(2000),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for GpioState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2000
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
