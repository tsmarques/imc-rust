use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[derive(Default)]
pub struct PushEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,
}

impl Message for PushEntityParameters {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PushEntityParameters {
            header: Header::new(811),

            _name: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PushEntityParameters {
            header: hdr,

            _name: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        811
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        811
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);
    }
}
