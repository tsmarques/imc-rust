use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use crate::Header::Header;

/// Request the state of power channels.
#[derive(Default)]
pub struct QueryPowerChannelState {
    /// IMC Header
    pub header: Header,
}

impl Message for QueryPowerChannelState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = QueryPowerChannelState {
            header: Header::new(310),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = QueryPowerChannelState { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        310
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        310
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
