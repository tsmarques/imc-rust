use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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
    pub fn as_primitive(&self) -> u32 {
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
pub struct LcdControl {
    /// IMC Header
    pub header: Header,

    /// The LCD action to perform
    pub _op: u8,

    /// Text to be written (if defined write operation).
    pub _text: String,
}

impl LcdControl {
    pub fn new() -> LcdControl {
        let mut msg = LcdControl {
            header: Header::new(307),

            _op: Default::default(),
            _text: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for LcdControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        307
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

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._text.as_bytes());

        serialize_footer(bfr);
    }
}
