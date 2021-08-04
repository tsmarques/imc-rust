use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

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

impl UamTxRange {
    pub fn new() -> UamTxRange {
        let mut msg = UamTxRange {
            header: Header::new(818),

            _seq: Default::default(),
            _sys_dst: Default::default(),
            _timeout: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for UamTxRange {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        818
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._sys_dst = Default::default();

        self._timeout = Default::default();
    }

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
}
