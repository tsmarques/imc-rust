use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = ClockControl {
            header: Header::new(106),

            _op: Default::default(),
            _clock: Default::default(),
            _tz: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = ClockControl {
            header: hdr,

            _op: Default::default(),
            _clock: Default::default(),
            _tz: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        106
    }

    #[inline(always)]
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

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._clock = bfr.get_f64_le();
        self._tz = bfr.get_i8();

        Ok(())
    }
}
