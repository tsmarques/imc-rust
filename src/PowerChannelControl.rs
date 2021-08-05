use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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
#[derive(Default)]
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
            header: Header::new(309),

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
        309
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._op = Default::default();

        self._sched_time = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        9
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._sched_time);
    }
}
