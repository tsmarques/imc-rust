use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 153;

/// Measure of the RSSI by a networking device.
/// Indicates the gain or loss in the signal strength due to the transmission and reception equipment and the transmission medium and distance.
pub struct RSSI {
    /// IMC Header
    pub header: Header,

    /// RSSI measurement.
    pub _value: f32,
}

impl RSSI {
    pub fn new() -> RSSI {
        let mut msg = RSSI {
            header: Header::new(c_msg_id),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for RSSI {
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

        bfr.put_f32_le(self._value);

        serialize_footer(bfr);
    }
}
