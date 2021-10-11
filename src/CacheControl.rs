use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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

impl Message for CacheControl {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = CacheControl {
            header: Header::new(101),

            _op: Default::default(),
            _snapshot: Default::default(),
            _message: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = CacheControl {
            header: hdr,

            _op: Default::default(),
            _snapshot: Default::default(),
            _message: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        101
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        101
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._snapshot = Default::default();

        self._message = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._snapshot.len() + 2;

        inline_message_serialization_size!(dyn_size, self._message);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._snapshot.as_bytes());
        serialize_inline_message!(bfr, self._message);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._op = bfr.get_u8();

        deserialize_string!(bfr, self._snapshot);

        self._message = deserialize_inline(bfr).ok();
    }
}
