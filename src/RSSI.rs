#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Measure of the RSSI by a networking device.
/// Indicates the gain or loss in the signal strength due to the transmission and reception equipment and the transmission medium and distance.
#[derive(Default)]
pub struct RSSI {
    /// IMC Header
    pub header: Header,

    /// RSSI measurement.
    pub _value: f32,
}

impl RSSI {
    pub fn new() -> RSSI {
        let mut msg = RSSI {
            header: Header::new(153),

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
        153
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
    }
}
