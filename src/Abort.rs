use crate::Message::*;

use crate::Header::Header;

/// Stops any executing actions and put the system in a standby mode.
#[derive(Default)]
pub struct Abort {
    /// IMC Header
    pub header: Header,
}

impl Message for Abort {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = Abort {
            header: Header::new(550),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = Abort { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        550
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        550
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
