use crate::Message::*;

use crate::Header::Header;

/// Command to obtain the operational limits in use by the vehicle.
#[derive(Default)]
pub struct GetOperationalLimits {
    /// IMC Header
    pub header: Header,
}

impl Message for GetOperationalLimits {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = GetOperationalLimits {
            header: Header::new(505),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = GetOperationalLimits { header: hdr };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        505
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        505
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
