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

impl Message for PulseDetectionControl {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = PulseDetectionControl {
            header: hdr,

            _op: Default::default(),
        };

        msg.get_header()._mgid = 278;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = PulseDetectionControl {
            header: Header::new(278),

            _op: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        278
    }

    fn id(&self) -> u16 {
        278
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
