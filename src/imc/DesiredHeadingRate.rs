use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 408;

/// Desired Heading Rate speed reference value for the control layer.
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

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

        bfr.put_f64_le(self._value);

        serialize_footer(bfr);
    }
}
