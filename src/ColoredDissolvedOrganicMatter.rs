#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Colored Dissolved Organic Matter measurement.
#[derive(Default)]
pub struct ColoredDissolvedOrganicMatter {
    /// IMC Header
    pub header: Header,

    /// Colored Dissolved Organic Matter reading.
    pub _value: f32,
}

impl ColoredDissolvedOrganicMatter {
    pub fn new() -> ColoredDissolvedOrganicMatter {
        let mut msg = ColoredDissolvedOrganicMatter {
            header: Header::new(2003),

            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ColoredDissolvedOrganicMatter {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        2003
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
