#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// Control torques allocated to the actuators.
#[derive(Default)]
pub struct AllocatedControlTorques {
    /// IMC Header
    pub header: Header,

    /// Torque K about the vehicle's x axis.
    pub _k: f64,

    /// Torque M about the vehicle's y axis.
    pub _m: f64,

    /// Torque N about the vehicle's z axis.
    pub _n: f64,
}

impl AllocatedControlTorques {
    pub fn new() -> AllocatedControlTorques {
        let mut msg = AllocatedControlTorques {
            header: Header::new(411),

            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for AllocatedControlTorques {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        411
    }

    fn clear(&mut self) {
        self.header.clear();

        self._k = Default::default();

        self._m = Default::default();

        self._n = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        24
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._k);
        bfr.put_f64_le(self._m);
        bfr.put_f64_le(self._n);
    }
}
