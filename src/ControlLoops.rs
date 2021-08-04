#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum EnableEnum {
    // Disable
    CL_DISABLE = 0,
    // Enable
    CL_ENABLE = 1,
}

impl EnableEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            CL_DISABLE => 0,
            CL_ENABLE => 1,
        }
    }
}

/// Enable or disable control loops.
#[derive(Default)]
pub struct ControlLoops {
    /// IMC Header
    pub header: Header,

    pub _enable: u8,

    /// Control loop mask.
    pub _mask: u32,

    /// Unsigned integer reference for the scope of the control loop message.
    /// Scope reference should only be set by a maneuver.
    /// Should be set to an always increasing reference at the time of dispatching this message.
    /// Lower level controllers must inherit the same scope reference sent by maneuver.
    /// This same scope reference must be sent down to lower control layers.
    pub _scope_ref: u32,
}

impl ControlLoops {
    pub fn new() -> ControlLoops {
        let mut msg = ControlLoops {
            header: Header::new(507),

            _enable: Default::default(),
            _mask: Default::default(),
            _scope_ref: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ControlLoops {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        507
    }

    fn clear(&mut self) {
        self.header.clear();

        self._enable = Default::default();

        self._mask = Default::default();

        self._scope_ref = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._enable);
        bfr.put_u32_le(self._mask);
        bfr.put_u32_le(self._scope_ref);
    }
}
