use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

/// Request information about an entity identifier. The receiving
/// system shall reply with an EntityInfo message with the details
/// of that entity.
#[derive(Default)]
pub struct QueryEntityInfo {
    /// IMC Header
    pub header: Header,

    /// Entity identifier.
    pub _id: u8,
}

impl Message for QueryEntityInfo {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityInfo {
            header: Header::new(4),

            _id: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityInfo {
            header: hdr,

            _id: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        4
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        4
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._id = bfr.get_u8();
    }
}
