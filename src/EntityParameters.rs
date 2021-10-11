use crate::Message::*;

use crate::MessageList;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::EntityParameter::EntityParameter;

use crate::packet::*;

/// List of entity parameters.
#[derive(Default)]
pub struct EntityParameters {
    /// IMC Header
    pub header: Header,

    /// Name of the entity.
    pub _name: String,

    /// List of parameters.
    pub _params: MessageList<EntityParameter>,
}

impl Message for EntityParameters {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EntityParameters {
            header: Header::new(802),

            _name: Default::default(),
            _params: vec![],
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EntityParameters {
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
        802
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        802
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);

        for m in self._params.iter_mut() {
            m.deserialize_fields(bfr);
        }
    }
}
