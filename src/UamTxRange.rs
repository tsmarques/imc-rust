use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

/// Request an acoustic modem driver to measure the distance to another system.
#[derive(Default)]
pub struct UamTxRange {
    /// IMC Header
    pub header: Header,

    /// A sequence identifier that should be incremented for each
    /// request. This number will then be used to issue transmission
    /// status updates via the message UamTxStatus.
    pub _seq: u16,

    /// The canonical name of the target system.
    pub _sys_dst: String,

    /// Maximum amount of time to wait for a reply.
    pub _timeout: f32,
}

impl Message for UamTxRange {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UamTxRange {
            header: Header::new(818),

            _seq: Default::default(),
            _sys_dst: Default::default(),
            _timeout: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UamTxRange {
            header: hdr,

            _seq: Default::default(),
            _sys_dst: Default::default(),
            _timeout: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        818
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        818
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._sys_dst = Default::default();

        self._timeout = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys_dst.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_f32_le(self._timeout);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._seq = bfr.get_u16_le();

        deserialize_string!(bfr, self._sys_dst);

        self._timeout = bfr.get_f32_le();
    }
}
