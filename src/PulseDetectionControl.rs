use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Pulse Detection OFF
    POP_OFF = 0,
    // Pulse Detection ON
    POP_ON = 1,
}

impl OperationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            POP_OFF => 0,
            POP_ON => 1,
        }
    }
}

/// Control of hardware pulse detection.
#[derive(Default)]
pub struct PulseDetectionControl {
    /// IMC Header
    pub header: Header,

    /// Activate or deactivate hardware pulse detection.
    pub _op: u8,
}

impl PulseDetectionControl {
    pub fn new() -> PulseDetectionControl {
        let mut msg = PulseDetectionControl {
            header: Header::new(278),

            _op: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PulseDetectionControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        278
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
    }
}
