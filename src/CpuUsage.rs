use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

/// Report of software CPU usage.
#[derive(Default)]
pub struct CpuUsage {
    /// IMC Header
    pub header: Header,

    /// The CPU usage, in percentage, of the sending software.
    pub _value: u8,
}

impl Message for CpuUsage {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = CpuUsage {
            header: Header::new(7),

            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = CpuUsage {
            header: hdr,

            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        7
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        7
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._value = bfr.get_u8();
    }
}
