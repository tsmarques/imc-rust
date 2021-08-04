#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Component of incremetal velocity vector.
#[derive(Default)]
pub struct VelocityDelta {
    /// IMC Header
    pub header: Header,

    /// The device time.
    pub _time: f64,

    /// X component.
    pub _x: f64,

    /// Y component.
    pub _y: f64,

    /// Z component.
    pub _z: f64,
}

impl VelocityDelta {
    pub fn new() -> VelocityDelta {
        let mut msg = VelocityDelta {
            header: Header::new(261),

            _time: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for VelocityDelta {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        261
    }

    fn clear(&mut self) {
        self.header.clear();

        self._time = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        32
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }
}
