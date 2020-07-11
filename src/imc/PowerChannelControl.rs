use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use crate::imc::Header::Header;
use bytes::BufMut;

const c_msg_id: u16 = 309;

pub enum OperationEnum {
    // Turn Off
    PCC_OP_TURN_OFF = 0,
    // Turn On
    PCC_OP_TURN_ON = 1,
    // Toggle
    PCC_OP_TOGGLE = 2,
    // Schedule Turn On
    PCC_OP_SCHED_ON = 3,
    // Schedule Turn Off
    PCC_OP_SCHED_OFF = 4,
    // Reset Schedules
    PCC_OP_SCHED_RESET = 5,
    // Save Current State
    PCC_OP_SAVE = 6,
}

impl OperationEnum {
    pub fn as_u32(&self) -> u32 {
        match self {
            PCC_OP_TURN_OFF => 0,
            PCC_OP_TURN_ON => 1,
            PCC_OP_TOGGLE => 2,
            PCC_OP_SCHED_ON => 3,
            PCC_OP_SCHED_OFF => 4,
            PCC_OP_SCHED_RESET => 5,
            PCC_OP_SAVE => 6,
        }
    }
}

/// Save the current state of the channel 'id' to persistent
/// storage.
pub struct PowerChannelControl {
    /// IMC Header
    pub header: Header,

    /// The name of the power channel.
    pub _name: String,

    /// Reset all scheduled operations for the channel specified in
    /// field 'id'.
    pub _op: u8,

    /// Scheduled time of operation.
    pub _sched_time: f64,
}

impl PowerChannelControl {
    pub fn new() -> PowerChannelControl {
        let mut msg = PowerChannelControl {
            header: Header::new(c_msg_id),

            _name: Default::default(),
            _op: Default::default(),
            _sched_time: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PowerChannelControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        c_msg_id
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._op = Default::default();

        self._sched_time = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        0
    }

    fn dynamic_serialization_size(&self) -> usize {
        unimplemented!();
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._sched_time);

        serialize_footer(bfr);
    }
}
