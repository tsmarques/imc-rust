use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum OperationEnum {
    // Execute Sync.
    COP_SYNC_EXEC = 0,
    // Request Sync.
    COP_SYNC_REQUEST = 1,
    // Sync. Started
    COP_SYNC_STARTED = 2,
    // Sync. done
    COP_SYNC_DONE = 3,
    // Set Timezone
    COP_SET_TZ = 4,
    // Timezone Setup
    COP_SET_TZ_DONE = 5,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            COP_SYNC_EXEC => 0,
            COP_SYNC_REQUEST => 1,
            COP_SYNC_STARTED => 2,
            COP_SYNC_DONE => 3,
            COP_SET_TZ => 4,
            COP_SET_TZ_DONE => 5,
        }
    }
}

/// Set timezone.
pub struct ClockControl {
    /// IMC Header
    pub header: Header,

    /// Notification due to timezone modification.
    pub _op: u8,

    /// Clock value (Epoch time).
    pub _clock: f64,

    /// Timezone.
    pub _tz: i8,
}

impl ClockControl {
    pub fn new() -> ClockControl {
        let mut msg = ClockControl {
            header: Header::new(106),

            _op: Default::default(),
            _clock: Default::default(),
            _tz: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for ClockControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        106
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._clock = Default::default();

        self._tz = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        10
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        bfr.put_f64_le(self._clock);
        bfr.put_i8(self._tz);

        serialize_footer(bfr);
    }
}
