use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

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
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = StorageUsage {
            header: hdr,

            _available: Default::default(),
            _value: Default::default(),
        };

        msg.get_header()._mgid = 100;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = StorageUsage {
            header: Header::new(100),

            _available: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        100
    }

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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
