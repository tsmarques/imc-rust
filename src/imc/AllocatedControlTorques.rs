use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 411;

/// Control torques allocated to the actuators.
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
            header: Header::new(c_msg_id),

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
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._k = Default::default();

        self._m = Default::default();

        self._n = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f64_le(self._k);
        bfr.put_f64_le(self._m);
        bfr.put_f64_le(self._n);

        serialize_footer(bfr);
    }
}
