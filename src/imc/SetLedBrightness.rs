use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 314;

/// Control the brightness of an LED (Light-Emitting Diode). The
/// recipient of this message shall set the intensity of the LED to
/// the desired 'value' and reply with 'LedBrightness'.
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

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

        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._value);

        serialize_footer(bfr);
    }
}
