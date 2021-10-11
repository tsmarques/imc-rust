use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Request a list of known underwater acoustic systems. The
/// recipient of this message shall reply with an AcousticSystems
/// message.
#[derive(Default)]
pub struct AcousticSystemsQuery {
    /// IMC Header
    pub header: Header,
}

impl Message for AcousticSystemsQuery {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = AcousticSystemsQuery {
            header: Header::new(212),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = AcousticSystemsQuery { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        212
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        212
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        Ok(())
    }
}
