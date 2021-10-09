use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct MessagePart {
    /// IMC Header
    pub header: Header,

    pub _uid: u8,

    pub _frag_number: u8,

    pub _num_frags: u8,

    pub _data: Vec<u8>,
}

impl Message for MessagePart {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = MessagePart {
            header: Header::new(877),

            _uid: Default::default(),
            _frag_number: Default::default(),
            _num_frags: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = MessagePart {
            header: hdr,

            _uid: Default::default(),
            _frag_number: Default::default(),
            _num_frags: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        877
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        877
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._uid = Default::default();

        self._frag_number = Default::default();

        self._num_frags = Default::default();

        self._data = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._uid);
        bfr.put_u8(self._frag_number);
        bfr.put_u8(self._num_frags);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._uid = bfr.get_u8();

        self._frag_number = bfr.get_u8();

        self._num_frags = bfr.get_u8();

        deserialize_bytes!(bfr, self._data);
    }
}
