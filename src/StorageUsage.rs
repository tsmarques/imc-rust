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

impl StorageUsage {
    pub fn new() -> StorageUsage {
        let mut msg = StorageUsage {
            header: Header::new(100),

            _available: Default::default(),
            _value: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for StorageUsage {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        100
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
}
