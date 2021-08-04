#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Initiate overall calibration of a vehicle.
#[derive(Default)]
pub struct Calibration {
    /// IMC Header
    pub header: Header,

    /// Duration of calibration.
    pub _duration: u16,
}

impl Calibration {
    pub fn new() -> Calibration {
        let mut msg = Calibration {
            header: Header::new(506),

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
        506
    }

    fn clear(&mut self) {
        self.header.clear();

        self._duration = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._duration);
    }
}
