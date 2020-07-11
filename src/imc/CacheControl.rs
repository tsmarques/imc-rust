use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 101;

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
    pub fn as_u32(&self) -> u32 {
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
            header: Header::new(c_msg_id),

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
        c_msg_id
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
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._snapshot.as_bytes());
        match &self._message {
            Some(field) => field.serialize(bfr),

            None => {}
        };

        serialize_footer(bfr);
    }
}
