#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

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
    pub fn as_primitive(&self) -> u32 {
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

impl MonitorEntityState {
    pub fn new() -> MonitorEntityState {
        let mut msg = MonitorEntityState {
            header: Header::new(502),

            _command: Default::default(),
            _entities: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for MonitorEntityState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        502
    }

    fn clear(&mut self) {
        self.header.clear();

        self._command = Default::default();

        self._entities = Default::default();
    }

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
}
