#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

/// This message describes an entity.
#[derive(Default)]
pub struct EntityInfo {
    /// IMC Header
    pub header: Header,

    /// Entity identifier.
    pub _id: u8,

    /// Entity label or empty if the entity id is not valid.
    pub _label: String,

    /// Name of the plugin/component/subsystem associated with this
    /// entity.
    pub _component: String,

    /// Amount of time needed to properly activate the entity.
    pub _act_time: u16,

    /// Amount of time needed to properly deactivate the entity.
    pub _deact_time: u16,
}

impl EntityInfo {
    pub fn new() -> EntityInfo {
        let mut msg = EntityInfo {
            header: Header::new(3),

            _id: Default::default(),
            _label: Default::default(),
            _component: Default::default(),
            _act_time: Default::default(),
            _deact_time: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for EntityInfo {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        3
    }

    fn clear(&mut self) {
        self.header.clear();

        self._id = Default::default();

        self._label = Default::default();

        self._component = Default::default();

        self._act_time = Default::default();

        self._deact_time = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        5
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._label.len() + 2;

        dyn_size += self._component.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        serialize_bytes!(bfr, self._label.as_bytes());
        serialize_bytes!(bfr, self._component.as_bytes());
        bfr.put_u16_le(self._act_time);
        bfr.put_u16_le(self._deact_time);
    }
}
