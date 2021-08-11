use crate::Message::*;

use crate::Header::Header;

/// Query the activation/deactivation state of an entity. The
/// recipient shall reply with an EntityActivationState message.
#[derive(Default)]
pub struct QueryEntityActivationState {
    /// IMC Header
    pub header: Header,
}

impl Message for QueryEntityActivationState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityActivationState {
            header: Header::new(15),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityActivationState { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        15
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        15
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
