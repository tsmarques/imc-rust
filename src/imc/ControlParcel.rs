use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 412;

/// Report of PID control parcels.
pub struct ControlParcel {
    /// IMC Header
    pub header: Header,

    /// Proportional parcel value.
    pub _p: f32,

    /// Integral parcel value.
    pub _i: f32,

    /// Derivative parcel value.
    pub _d: f32,

    /// Anti-windup parcel value.
    pub _a: f32,
}

impl ControlParcel {
    pub fn new() -> ControlParcel {
        let mut msg = ControlParcel {
            header: Header::new(c_msg_id),

            _p: Default::default(),
            _i: Default::default(),
            _d: Default::default(),
            _a: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ControlParcel {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._p = Default::default();

        self._i = Default::default();

        self._d = Default::default();

        self._a = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._i);
        bfr.put_f32_le(self._d);
        bfr.put_f32_le(self._a);

        serialize_footer(bfr);
    }
}
