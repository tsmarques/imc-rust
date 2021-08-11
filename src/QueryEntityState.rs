use crate::Message::*;

use crate::Header::Header;

/// Request entities to report their state. Entities should respond
/// by issuing an appropriate EntityState message.
#[derive(Default)]
pub struct QueryEntityState {
    /// IMC Header
    pub header: Header,
}

impl Message for QueryEntityState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityState {
            header: Header::new(2),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityState { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        2
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {}

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
