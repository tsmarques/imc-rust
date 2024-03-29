use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum StateEnum {
    // Off
    PCS_OFF = 0,
    // On
    PCS_ON = 1,
}

impl StateEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            PCS_OFF => 0,
            PCS_ON => 1,
        }
    }
}

/// Power channel is off.
pub struct PowerChannelState {
    /// IMC Header
    pub header: Header,

    /// Power Channel Name.
    pub _name: String,

    /// Power channel is on.
    pub _state: u8,
}

impl PowerChannelState {
    pub fn new() -> PowerChannelState {
        let mut msg = PowerChannelState {
            header: Header::new(311),

            _name: Default::default(),
            _state: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PowerChannelState {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        311
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._state = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._state);

        serialize_footer(bfr);
    }
}
