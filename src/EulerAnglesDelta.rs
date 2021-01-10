use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Component of incremetal orientation vector over a period of time.
pub struct EulerAnglesDelta {
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

    /// Period of time of the orientation vector increments.
    pub _timestep: f32,
}

impl EulerAnglesDelta {
    pub fn new() -> EulerAnglesDelta {
        let mut msg = EulerAnglesDelta {
            header: Header::new(255),

            _time: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _timestep: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EulerAnglesDelta {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        255
    }

    fn clear(&mut self) {
        self.header.clear();

        self._time = Default::default();

        self._x = Default::default();

        self._y = Default::default();

        self._z = Default::default();

        self._timestep = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        36
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
        bfr.put_f32_le(self._timestep);

        serialize_footer(bfr);
    }
}
