use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum operationEnum {
    // Report
    OP_REPORT = 0,
    // Query
    OP_QUERY = 1,
}

#[allow(non_camel_case_types)]
pub mod operation {
    // Report
    pub const OP_REPORT: u32 = 0;
    // Query
    pub const OP_QUERY: u32 = 1;
}

/// This message describes the names and identification numbers of
/// all entities in the system.
#[derive(Default)]
pub struct EntityList {
    /// IMC Header
    pub header: Header,

    /// Operation to perform.
    pub _op: u8,

    /// Example: &quot;Battery=11;CTD=3&quot;
    pub _list: String,
}

impl Message for EntityList {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EntityList {
            header: Header::new(5),

            _op: Default::default(),
            _list: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EntityList {
            header: hdr,

            _op: Default::default(),
            _list: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        5
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        5
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
        self._list = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._list.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._list.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._list);

        Ok(())
    }
}
