use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum SanityEnum {
    // Sane
    DS_SANE = 0,
    // Not Sane
    DS_NOT_SANE = 1,
}

impl SanityEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            DS_SANE => 0,
            DS_NOT_SANE => 1,
        }
    }
}

/// Data is sane.
#[derive(Default)]
pub struct DataSanity {
    /// IMC Header
    pub header: Header,

    /// Data is not sane.
    pub _sane: u8,
}

impl Message for DataSanity {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = DataSanity {
            header: hdr,

            _sane: Default::default(),
        };

        msg.get_header()._mgid = 284;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = DataSanity {
            header: Header::new(284),

            _sane: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        284
    }

    fn id(&self) -> u16 {
        284
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._sane = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._sane);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
