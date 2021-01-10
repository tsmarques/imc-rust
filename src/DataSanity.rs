use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum SanityEnum {
    // Sane
    DS_SANE = 0,
    // Not Sane
    DS_NOT_SANE = 1,
}

impl SanityEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            DS_SANE => 0,
            DS_NOT_SANE => 1,
        }
    }
}

/// Data is sane.
pub struct DataSanity {
    /// IMC Header
    pub header: Header,

    /// Data is not sane.
    pub _sane: u8,
}

impl DataSanity {
    pub fn new() -> DataSanity {
        let mut msg = DataSanity {
            header: Header::new(284),

            _sane: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for DataSanity {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        284
    }

    fn clear(&mut self) {
        self.header.clear();

        self._sane = Default::default();
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

        bfr.put_u8(self._sane);

        serialize_footer(bfr);
    }
}
