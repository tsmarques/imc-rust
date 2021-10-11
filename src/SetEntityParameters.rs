use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::EntityParameter::EntityParameter;

use crate::packet::ImcError;
use crate::packet::*;

#[derive(Default)]
pub struct SetEntityParameters {
    /// IMC Header
    pub header: Header,

    pub _name: String,

    pub _params: MessageList<EntityParameter>,
}

impl Message for SetEntityParameters {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = SetEntityParameters {
            header: Header::new(804),

            _name: Default::default(),
            _params: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = SetEntityParameters {
            header: hdr,

            _name: Default::default(),
            _params: vec![],
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        804
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        804
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._params = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        message_list_serialization_size!(dyn_size, self._params);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_message_list!(bfr, self._params);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);

        self._params = deserialize_message_list_as::<EntityParameter>(bfr)?;

        Ok(())
    }
}
