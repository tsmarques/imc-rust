use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

pub enum OperationEnum {
    // Power Down
    POP_PWR_DOWN = 0,
    // Power Down in Progress
    POP_PWR_DOWN_IP = 1,
    // Power Down Aborted
    POP_PWR_DOWN_ABORTED = 2,
    // Schedule Power Down
    POP_SCHED_PWR_DOWN = 3,
    // Power Up
    POP_PWR_UP = 4,
    // Power Up in Progress
    POP_PWR_UP_IP = 5,
    // Schedule Power Up
    POP_SCHED_PWR_UP = 6,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            POP_PWR_DOWN => 0,
            POP_PWR_DOWN_IP => 1,
            POP_PWR_DOWN_ABORTED => 2,
            POP_SCHED_PWR_DOWN => 3,
            POP_PWR_UP => 4,
            POP_PWR_UP_IP => 5,
            POP_SCHED_PWR_UP => 6,
        }
    }
}

/// Request the destination entity of this message to power up
/// it's devices at the time given in the field 'sched_time'. If
/// the destination entity is the special entity '0' the whole
/// system will power up.
pub struct PowerOperation {
    /// IMC Header
    pub header: Header,

    /// The latest power up request is in progress.
    pub _op: u8,

    /// Time remaining to complete operation.
    pub _time_remain: f32,

    /// Scheduled time of operation.
    pub _sched_time: f64,
}

impl PowerOperation {
    pub fn new() -> PowerOperation {
        let mut msg = PowerOperation {
            header: Header::new(308),

            _op: Default::default(),
            _time_remain: Default::default(),
            _sched_time: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PowerOperation {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        308
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._time_remain = Default::default();

        self._sched_time = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        13
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        bfr.put_f32_le(self._time_remain);
        bfr.put_f64_le(self._sched_time);

        serialize_footer(bfr);
    }
}
