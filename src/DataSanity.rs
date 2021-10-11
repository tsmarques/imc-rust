use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = DataSanity {
            header: Header::new(284),

            _sane: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = DataSanity {
            header: hdr,

            _sane: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        284
    }

    #[inline(always)]
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

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._sane);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._sane = bfr.get_u8();
    }
}
