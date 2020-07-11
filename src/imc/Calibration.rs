use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 506;

/// Initiate overall calibration of a vehicle.
pub struct Calibration {
    /// IMC Header
    pub header: Header,

    /// Duration of calibration.
    pub _duration: u16,
}

impl Calibration {
    pub fn new() -> Calibration {
        let mut msg = Calibration {
            header: Header::new(c_msg_id),

            _duration: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for Calibration {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._duration = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._duration);

        serialize_footer(bfr);
    }
}
