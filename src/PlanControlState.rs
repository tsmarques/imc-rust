use crate::Message::*;

use bytes::BufMut;

use crate::Header::Header;

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

impl StateEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            PCS_BLOCKED => 0,
            PCS_READY => 1,
            PCS_INITIALIZING => 2,
            PCS_EXECUTING => 3,
        }
    }
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

impl LastPlanOutcomeEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            LPO_NONE => 0,
            LPO_SUCCESS => 1,
            LPO_FAILURE => 2,
        }
    }
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
    fn from(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let mut msg = PlanControlState {
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

        msg.get_header()._mgid = 560;
        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        let mut msg = PlanControlState {
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

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }

    fn static_id() -> u16
    where
        Self: Sized,
    {
        560
    }

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

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) {}
}
