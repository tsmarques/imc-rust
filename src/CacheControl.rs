use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum ControlOperationEnum {
    // Store
    COP_STORE = 0,
    // Load
    COP_LOAD = 1,
    // Clear
    COP_CLEAR = 2,
    // Copy Snapshot
    COP_COPY = 3,
    // Snapshot Copy Complete
    COP_COPY_COMPLETE = 4,
}

impl ControlOperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            COP_STORE => 0,
            COP_LOAD => 1,
            COP_CLEAR => 2,
            COP_COPY => 3,
            COP_COPY_COMPLETE => 4,
        }
    }
}

/// Control caching of messages to persistent storage.
#[derive(Default)]
pub struct CacheControl {
    /// IMC Header
    pub header: Header,

    /// Operation to perform.
    pub _op: u8,

    /// Destination for the cache snapshot file.
    pub _snapshot: String,

    /// Message to store.
    pub _message: Option<Box<dyn Message>>,
}

impl CacheControl {
    pub fn new() -> CacheControl {
        let mut msg = CacheControl {
            header: Header::new(101),

            _op: Default::default(),
            _snapshot: Default::default(),
            _message: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for CacheControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        101
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._snapshot = Default::default();

        match &mut self._message {
            Some(field) => field.clear(),

            None => {}
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._snapshot.len() + 2;

        match &self._message {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._snapshot.as_bytes());
        match &self._message {
            None => {}

            Some(m) => m.serialize_fields(bfr),
        };
    }
}
