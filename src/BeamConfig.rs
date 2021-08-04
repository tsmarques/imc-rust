#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Beam configuration of the device.
#[derive(Default)]
pub struct BeamConfig {
    /// IMC Header
    pub header: Header,

    /// Beam width of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_width: f32,

    /// Beam height of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_height: f32,
}

impl BeamConfig {
    pub fn new() -> BeamConfig {
        let mut msg = BeamConfig {
            header: Header::new(283),

            _beam_width: Default::default(),
            _beam_height: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for BeamConfig {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        283
    }

    fn clear(&mut self) {
        self.header.clear();

        self._beam_width = Default::default();

        self._beam_height = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._beam_width);
        bfr.put_f32_le(self._beam_height);
    }
}
