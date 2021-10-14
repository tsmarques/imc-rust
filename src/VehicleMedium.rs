use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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

/// Vehicle medium is unknown
#[derive(Default)]
pub struct VehicleMedium {
    /// IMC Header
    pub header: Header,

    /// Vehicle is underwater
    pub _medium: u8,
}

impl Message for VehicleMedium {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = VehicleMedium {
            header: Header::new(508),

            _medium: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = VehicleMedium {
            header: hdr,

            _medium: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        508
    }

    #[inline(always)]
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

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._medium);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._medium = bfr.get_u8();

        Ok(())
    }
}
