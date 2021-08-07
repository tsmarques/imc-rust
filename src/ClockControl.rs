use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
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
#[derive(Default)]
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

impl Message for ClockControl {
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = ClockControl {
            header: hdr,

            _op: Default::default(),
            _clock: Default::default(),
            _tz: Default::default(),
        };

        msg.get_header()._mgid = 106;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = ClockControl {
            header: Header::new(106),

            _op: Default::default(),
            _clock: Default::default(),
            _tz: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        106
    }

    fn id(&self) -> u16 {
        106
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
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
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._clock);
        bfr.put_i8(self._tz);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
