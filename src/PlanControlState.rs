use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum StateEnum {
    // Blocked
    PCS_BLOCKED = 0,
    // Ready
    PCS_READY = 1,
    // Initializing
    PCS_INITIALIZING = 2,
    // Executing
    PCS_EXECUTING = 3,
}

#[allow(non_camel_case_types)]
pub enum LastPlanOutcomeEnum {
    // None
    LPO_NONE = 0,
    // Success
    LPO_SUCCESS = 1,
    // Failure
    LPO_FAILURE = 2,
}

#[allow(non_camel_case_types)]
pub mod State {
    // Blocked
    pub const PCS_BLOCKED: u32 = 0;
    // Ready
    pub const PCS_READY: u32 = 1;
    // Initializing
    pub const PCS_INITIALIZING: u32 = 2;
    // Executing
    pub const PCS_EXECUTING: u32 = 3;
}

#[allow(non_camel_case_types)]
pub mod LastPlanOutcome {
    // None
    pub const LPO_NONE: u32 = 0;
    // Success
    pub const LPO_SUCCESS: u32 = 1;
    // Failure
    pub const LPO_FAILURE: u32 = 2;
}

/// Last plan execution was a failure.
#[derive(Default)]
pub struct PlanControlState {
    /// IMC Header
    pub header: Header,

    /// Executing plan.
    pub _state: u8,

    /// Identifier of plan currently loaded.
    pub _plan_id: String,

    /// Current plan estimated time to completion.
    /// The value will be -1 if the time is unknown or undefined.
    pub _plan_eta: i32,

    /// Current plan estimated progress in percent.
    /// The value will be negative if unknown or undefined.
    pub _plan_progress: f32,

    /// Current node ID, when executing a plan.
    pub _man_id: String,

    /// Type of maneuver being executed (IMC serialization id),
    /// when executing a plan.
    pub _man_type: u16,

    /// Current node estimated time to completion, when executing a plan.
    /// The value will be -1 if the time is unknown or undefined.
    pub _man_eta: i32,

    /// Last plan execution was successful.
    pub _last_outcome: u8,
}

impl Message for PlanControlState {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanControlState {
            header: Header::new(560),

            _state: Default::default(),
            _plan_id: Default::default(),
            _plan_eta: Default::default(),
            _plan_progress: Default::default(),
            _man_id: Default::default(),
            _man_type: Default::default(),
            _man_eta: Default::default(),
            _last_outcome: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanControlState {
            header: hdr,

            _state: Default::default(),
            _plan_id: Default::default(),
            _plan_eta: Default::default(),
            _plan_progress: Default::default(),
            _man_id: Default::default(),
            _man_type: Default::default(),
            _man_eta: Default::default(),
            _last_outcome: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        560
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        560
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._state = Default::default();
        self._plan_id = Default::default();
        self._plan_eta = Default::default();
        self._plan_progress = Default::default();
        self._man_id = Default::default();
        self._man_type = Default::default();
        self._man_eta = Default::default();
        self._last_outcome = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        dyn_size += self._man_id.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_i32_le(self._plan_eta);
        bfr.put_f32_le(self._plan_progress);
        serialize_bytes!(bfr, self._man_id.as_bytes());
        bfr.put_u16_le(self._man_type);
        bfr.put_i32_le(self._man_eta);
        bfr.put_u8(self._last_outcome);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        deserialize_string!(bfr, self._plan_id);
        self._plan_eta = bfr.get_i32_le();
        self._plan_progress = bfr.get_f32_le();
        deserialize_string!(bfr, self._man_id);
        self._man_type = bfr.get_u16_le();
        self._man_eta = bfr.get_i32_le();
        self._last_outcome = bfr.get_u8();

        Ok(())
    }
}
