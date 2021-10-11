use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[derive(Default)]
pub struct QueryEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,

    pub _visibility: String,

    pub _scope: String,
}

impl Message for QueryEntityParameters {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityParameters {
            header: Header::new(803),

            _name: Default::default(),
            _visibility: Default::default(),
            _scope: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = QueryEntityParameters {
            header: hdr,

            _name: Default::default(),
            _visibility: Default::default(),
            _scope: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        803
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        803
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._visibility = Default::default();

        self._scope = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._visibility.len() + 2;

        dyn_size += self._scope.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._visibility.as_bytes());
        serialize_bytes!(bfr, self._scope.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);

        deserialize_string!(bfr, self._visibility);

        deserialize_string!(bfr, self._scope);

        Ok(())
    }
}
