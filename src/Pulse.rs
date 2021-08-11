use crate::Message::*;

use crate::Header::Header;

/// Hardware pulse detection.
#[derive(Default)]
pub struct Pulse {
    /// IMC Header
    pub header: Header,
}

impl Message for Pulse {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Pulse {
            header: Header::new(277),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Pulse { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        277
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        277
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
