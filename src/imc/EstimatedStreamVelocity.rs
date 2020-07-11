use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 351;

/// The estimated stream velocity, typically for water or air
/// streams.
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);

        serialize_footer(bfr);
    }
}
