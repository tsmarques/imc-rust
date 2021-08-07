use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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
#[derive(Default)]
pub struct VehicleMedium {
    /// IMC Header
    pub header: Header,

    /// Vehicle is underwater
    pub _medium: u8,
}

impl Message for VehicleMedium {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = VehicleMedium {
            header: hdr,

            _medium: Default::default(),
        };

        msg.get_header()._mgid = 508;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = VehicleMedium {
            header: Header::new(508),

            _medium: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        508
    }

    fn id(&self) -> u16 {
        508
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._medium = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._medium);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
