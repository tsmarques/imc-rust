use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct IridiumMsgTx {
    /// IMC Header
    pub header: Header,

    /// The request identifier used to receive transmission updates.
    pub _req_id: u16,

    /// Time, in seconds, after which there will be no more atempts to transmit the message.
    pub _ttl: u16,

    /// The unique identifier of this message's destination (e.g. lauv-xtreme-2, manta-0).
    pub _destination: String,

    /// Message data.
    pub _data: Vec<u8>,
}

impl Message for IridiumMsgTx {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = IridiumMsgTx {
            header: Header::new(171),

            _req_id: Default::default(),
            _ttl: Default::default(),
            _destination: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = IridiumMsgTx {
            header: hdr,

            _req_id: Default::default(),
            _ttl: Default::default(),
            _destination: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        171
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        171
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._req_id = Default::default();

        self._ttl = Default::default();

        self._destination = Default::default();

        self._data = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._destination.len() + 2;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u16_le(self._ttl);
        serialize_bytes!(bfr, self._destination.as_bytes());
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._req_id = bfr.get_u16_le();

        self._ttl = bfr.get_u16_le();

        deserialize_string!(bfr, self._destination);

        deserialize_bytes!(bfr, self._data);
    }
}
