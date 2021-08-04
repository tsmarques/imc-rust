#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// The estimated stream velocity, typically for water or air
/// streams.
#[derive(Default)]
pub struct EstimatedStreamVelocity {
    /// IMC Header
    pub header: Header,

    /// X component (North).
    pub _x: f64,

    /// Y component (East).
    pub _y: f64,

    /// Z component (Down).
    pub _z: f64,
}

impl EstimatedStreamVelocity {
    pub fn new() -> EstimatedStreamVelocity {
        let mut msg = EstimatedStreamVelocity {
            header: Header::new(351),

            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EstimatedStreamVelocity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        351
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }
}
