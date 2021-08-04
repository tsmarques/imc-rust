#![allow(non_snake_case)]

use crate::Message::*;
use crate::{MessageList, DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum StateEnum {
    // Maneuver in progress
    MCS_EXECUTING = 0,
    // Maneuver completed
    MCS_DONE = 1,
    // Maneuver error
    MCS_ERROR = 2,
    // Maneuver stopped
    MCS_STOPPED = 3,
}

impl StateEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            MCS_EXECUTING => 0,
            MCS_DONE => 1,
            MCS_ERROR => 2,
            MCS_STOPPED => 3,
        }
    }
}

/// Maneuver error.
#[derive(Default)]
pub struct ManeuverControlState {
    /// IMC Header
    pub header: Header,

    /// Maneuver stopped.
    pub _state: u8,

    /// Estimated time to completion of the maneuver, when executing.
    /// The value will be 65535 if the time is unknown or undefined.
    pub _eta: u16,

    /// Complementary information, e.g., regarding errors.
    pub _info: String,
}

impl ManeuverControlState {
    pub fn new() -> ManeuverControlState {
        let mut msg = ManeuverControlState {
            header: Header::new(470),

            _state: Default::default(),
            _eta: Default::default(),
            _info: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ManeuverControlState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        470
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();

        self._eta = Default::default();

        self._info = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        3
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        bfr.put_u16_le(self._eta);
        serialize_bytes!(bfr, self._info.as_bytes());
    }
}
