use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum MediumEnum {
    // Ground
    VM_GROUND = 0,
    // Air
    VM_AIR = 1,
    // Water
    VM_WATER = 2,
    // Underwater
    VM_UNDERWATER = 3,
    // Unknown
    VM_UNKNOWN = 4,
}

impl MediumEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            VM_GROUND => 0,
            VM_AIR => 1,
            VM_WATER => 2,
            VM_UNDERWATER => 3,
            VM_UNKNOWN => 4,
        }
    }
}

/// Vehicle medium is unknown
pub struct VehicleMedium {
    /// IMC Header
    pub header: Header,

    /// Vehicle is underwater
    pub _medium: u8,
}

impl VehicleMedium {
    pub fn new() -> VehicleMedium {
        let mut msg = VehicleMedium {
            header: Header::new(508),

            _medium: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for VehicleMedium {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        508
    }

    fn clear(&mut self) {
        self.header.clear();

        self._medium = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._medium);

        serialize_footer(bfr);
    }
}
