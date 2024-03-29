use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

/// This message contains information, collected using USBL, about the
/// bearing and elevation of a target.
pub struct UsblAngles {
    /// IMC Header
    pub header: Header,

    /// Target's IMC address.
    pub _target: u16,

    /// Target's bearing.
    pub _bearing: f32,

    /// Target's elevation.
    pub _elevation: f32,
}

impl UsblAngles {
    pub fn new() -> UsblAngles {
        let mut msg = UsblAngles {
            header: Header::new(890),

            _target: Default::default(),
            _bearing: Default::default(),
            _elevation: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UsblAngles {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        890
    }

    fn clear(&mut self) {
        self.header.clear();

        self._target = Default::default();

        self._bearing = Default::default();

        self._elevation = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        10
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u16_le(self._target);
        bfr.put_f32_le(self._bearing);
        bfr.put_f32_le(self._elevation);

        serialize_footer(bfr);
    }
}
