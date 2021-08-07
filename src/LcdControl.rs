use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Turn off display
    OP_TURN_OFF = 0,
    // Turn on display
    OP_TURN_ON = 1,
    // Clear display
    OP_CLEAR = 2,
    // Write Line #0
    OP_WRITE0 = 3,
    // Write Line #1
    OP_WRITE1 = 4,
}

impl OperationEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            OP_TURN_OFF => 0,
            OP_TURN_ON => 1,
            OP_CLEAR => 2,
            OP_WRITE0 => 3,
            OP_WRITE1 => 4,
        }
    }
}

/// Control LCD.
#[derive(Default)]
pub struct LcdControl {
    /// IMC Header
    pub header: Header,

    /// The LCD action to perform
    pub _op: u8,

    /// Text to be written (if defined write operation).
    pub _text: String,
}

impl Message for LcdControl {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = LcdControl {
            header: hdr,

            _op: Default::default(),
            _text: Default::default(),
        };

        msg.get_header()._mgid = 307;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = LcdControl {
            header: Header::new(307),

            _op: Default::default(),
            _text: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        307
    }

    fn id(&self) -> u16 {
        307
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._text = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
