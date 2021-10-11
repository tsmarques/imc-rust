use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

/// Report of storage usage.
#[derive(Default)]
pub struct StorageUsage {
    /// IMC Header
    pub header: Header,

    /// The available storage of the reporting device.
    pub _available: u32,

    /// The percentage of storage used by the reporting device.
    pub _value: u8,
}

impl Message for StorageUsage {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = StorageUsage {
            header: Header::new(100),

            _available: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = StorageUsage {
            header: hdr,

            _available: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        100
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        100
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._available = Default::default();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._available);
        bfr.put_u8(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._available = bfr.get_u32_le();

        self._value = bfr.get_u8();
    }
}
