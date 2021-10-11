use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

/// Entity parameter.
#[derive(Default)]
pub struct EntityParameter {
    /// IMC Header
    pub header: Header,

    /// Name of the parameter.
    pub _name: String,

    /// Current value of the parameter.
    pub _value: String,
}

impl Message for EntityParameter {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EntityParameter {
            header: Header::new(801),

            _name: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EntityParameter {
            header: hdr,

            _name: Default::default(),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        801
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        801
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._value = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._value.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);

        deserialize_string!(bfr, self._value);

        Ok(())
    }
}
