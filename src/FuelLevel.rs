#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Report of fuel level.
#[derive(Default)]
pub struct FuelLevel {
    /// IMC Header
    pub header: Header,

    /// Fuel level percentage of the system.
    pub _value: f32,

    /// Percentage level of confidence in the estimation of the amount
    /// of energy in the batteries.
    pub _confidence: f32,

    /// Operation mode name and the estimated time available in that
    /// mode in hours. Example: &quot;Motion=1.5&quot;
    pub _opmodes: String,
}

impl FuelLevel {
    pub fn new() -> FuelLevel {
        let mut msg = FuelLevel {
            header: Header::new(279),

            _value: Default::default(),
            _confidence: Default::default(),
            _opmodes: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for FuelLevel {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        279
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();

        self._confidence = Default::default();

        self._opmodes = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        8
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._opmodes.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_f32_le(self._confidence);
        serialize_bytes!(bfr, self._opmodes.as_bytes());
    }
}
