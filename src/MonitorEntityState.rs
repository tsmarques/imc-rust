use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
pub enum CommandEnum {
    // Reset to defaults
    MES_RESET = 0,
    // Enable Monitoring
    MES_ENABLE = 1,
    // Disable Monitoring
    MES_DISABLE = 2,
    // Enable Monitoring (exclusive - disables all others)
    MES_ENABLE_EXCLUSIVE = 3,
    // Status Report
    MES_STATUS = 4,
}

impl CommandEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            MES_RESET => 0,
            MES_ENABLE => 1,
            MES_DISABLE => 2,
            MES_ENABLE_EXCLUSIVE => 3,
            MES_STATUS => 4,
        }
    }
}

/// Controls monitoring of entity states in the vehicle.
#[derive(Default)]
pub struct MonitorEntityState {
    /// IMC Header
    pub header: Header,

    /// Command.
    pub _command: u8,

    /// Comma separated list of entity names.
    pub _entities: String,
}

impl Message for MonitorEntityState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = MonitorEntityState {
            header: Header::new(502),

            _command: Default::default(),
            _entities: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = MonitorEntityState {
            header: hdr,

            _command: Default::default(),
            _entities: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        502
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        502
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._command = Default::default();

        self._entities = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._entities.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._command);
        serialize_bytes!(bfr, self._entities.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        self._command = bfr.get_u8();

        deserialize_string!(bfr, self._entities);
    }
}
