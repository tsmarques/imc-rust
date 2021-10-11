use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Bootstrapping
    ESTA_BOOT = 0,
    // Normal Operation
    ESTA_NORMAL = 1,
    // Fault
    ESTA_FAULT = 2,
    // Error
    ESTA_ERROR = 3,
    // Failure
    ESTA_FAILURE = 4,
}

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            ESTA_BOOT => 0,
            ESTA_NORMAL => 1,
            ESTA_FAULT => 2,
            ESTA_ERROR => 3,
            ESTA_FAILURE => 4,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Human Intervention Required
    EFLA_HUMAN_INTERVENTION = 0x01,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            EFLA_HUMAN_INTERVENTION => 0x01,
        }
    }
}

/// State reported by an entity in the vehicle. The source entity is
/// identified in the message header.
#[derive(Default)]
pub struct EntityState {
    /// IMC Header
    pub header: Header,

    /// State of entity.
    pub _state: u8,

    /// Complementary entity state flags.
    pub _flags: u8,

    /// Complementary human-readable description of entity state.
    pub _description: String,
}

impl Message for EntityState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = EntityState {
            header: Header::new(1),

            _state: Default::default(),
            _flags: Default::default(),
            _description: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = EntityState {
            header: hdr,

            _state: Default::default(),
            _flags: Default::default(),
            _description: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        1
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        1
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();

        self._flags = Default::default();

        self._description = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._description.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._description.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._state = bfr.get_u8();

        self._flags = bfr.get_u8();

        deserialize_string!(bfr, self._description);
    }
}
