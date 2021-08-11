use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Acknowledgement
    UTF_ACK = 0x01,
    // Delayed
    UTF_DELAYED = 0x02,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            UTF_ACK => 0x01,
            UTF_DELAYED => 0x02,
        }
    }
}

/// On modems that support it, this flag shall be used to request an
/// acknowledgment of reception from the receiving node.
#[derive(Default)]
pub struct UamTxFrame {
    /// IMC Header
    pub header: Header,

    /// A sequence identifier that should be incremented for each
    /// request. This number will then be used to issue transmission
    /// status updates via the message UamTxStatus.
    pub _seq: u16,

    /// The canonical name of the destination system. If supported, the
    /// special destination 'broadcast' shall be used to dispatch messages
    /// to all nodes.
    pub _sys_dst: String,

    /// On modems that support it, this flag shall be used to delay the
    /// frame transmission until the modem needs to transmit control
    /// data (e.g., acknowledgment of reception, etc).
    pub _flags: u8,

    /// The actual data frame to transmit. The data size shall not exceed
    /// the MTU of the acoustic modem.
    pub _data: Vec<u8>,
}

impl Message for UamTxFrame {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = UamTxFrame {
            header: Header::new(814),

            _seq: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = UamTxFrame {
            header: hdr,

            _seq: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        814
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        814
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._seq = Default::default();

        self._sys_dst = Default::default();

        self._flags = Default::default();

        self._data = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._sys_dst.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._seq = bfr.get_u16_le();

        deserialize_string!(bfr, self._sys_dst);

        self._flags = bfr.get_u8();

        deserialize_bytes!(bfr, self._data);
    }
}
