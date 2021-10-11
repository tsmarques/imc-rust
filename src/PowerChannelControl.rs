use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::*;

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

impl Message for PowerChannelControl {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PowerChannelControl {
            header: Header::new(309),

            _name: Default::default(),
            _op: Default::default(),
            _sched_time: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PowerChannelControl {
            header: hdr,

            _name: Default::default(),
            _op: Default::default(),
            _sched_time: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        309
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        309
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._name = Default::default();

        self._op = Default::default();

        self._sched_time = Default::default();
    }

    #[inline(always)]
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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {
        deserialize_string!(bfr, self._name);

        self._op = bfr.get_u8();

        self._sched_time = bfr.get_f64_le();
    }
}
