use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

/// Event of a specific hardware button.
pub struct ButtonEvent {
    /// IMC Header
    pub header: Header,

    /// Button identifier.
    pub _button: u8,

    /// Value of the button.
    pub _value: u8,
}

impl ButtonEvent {
    pub fn new() -> ButtonEvent {
        let mut msg = ButtonEvent {
            header: Header::new(306),

            _button: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ButtonEvent {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        306
    }

    fn clear(&mut self) {
        self.header.clear();

        self._button = Default::default();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._button);
        bfr.put_u8(self._value);

        serialize_footer(bfr);
    }
}
