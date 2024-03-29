use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

pub enum TypeEnum {
    // Request
    PC_REQUEST = 0,
    // Reply -- Success
    PC_SUCCESS = 1,
    // Reply -- Failure
    PC_FAILURE = 2,
    // Reply -- In Progress
    PC_IN_PROGRESS = 3,
}

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            PC_REQUEST => 0,
            PC_SUCCESS => 1,
            PC_FAILURE => 2,
            PC_IN_PROGRESS => 3,
        }
    }
}

pub enum OperationEnum {
    // Start Plan
    PC_START = 0,
    // Stop Plan
    PC_STOP = 1,
    // Load Plan
    PC_LOAD = 2,
    // Get Plan
    PC_GET = 3,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            PC_START => 0,
            PC_STOP => 1,
            PC_LOAD => 2,
            PC_GET => 3,
        }
    }
}

pub enum FlagsEnum {
    // Calibrate Vehicle
    FLG_CALIBRATE = 0x0001,
    // Ignore Errors
    FLG_IGNORE_ERRORS = 0x0002,
}

impl FlagsEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            FLG_CALIBRATE => 0x0001,
            FLG_IGNORE_ERRORS => 0x0002,
        }
    }
}

/// Execute current plan while ignoring some errors that might be active.
pub struct PlanControl {
    /// IMC Header
    pub header: Header,

    /// Indicates if the message is a request or a reply to a
    /// previous request. The *op*, *request_id* and *plan_id* fields
    /// of a request will be echoed in one or more responses to that
    /// request.
    pub _type: u8,

    /// Get loaded plan. For a successful reply, the *data* field
    /// will contain the :ref:`PlanSpecification` message that
    /// corresponds to the currently loaded plan.
    pub _op: u8,

    /// Request ID. This may be used by interfacing modules e.g. using
    /// sequence counters.  to annotate requests and appropriately
    /// identify replies.
    pub _request_id: u16,

    /// The identifier for the plan to be stopped / started / loaded /
    /// retrieved according to the command requested (*op* field).
    pub _plan_id: String,

    /// Perform vehicle calibration.
    pub _flags: u16,

    /// Complementary message argument for requests/replies.
    pub _arg: Option<Box<dyn Message>>,

    /// Complementary human-readable information. This is used
    /// in association to replies.
    pub _info: String,
}

impl PlanControl {
    pub fn new() -> PlanControl {
        let mut msg = PlanControl {
            header: Header::new(559),

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _flags: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanControl {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        559
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._op = Default::default();

        self._request_id = Default::default();

        self._plan_id = Default::default();

        self._flags = Default::default();

        match &mut self._arg {
            Some(field) => field.clear(),

            None => {}
        }

        self._info = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        match &self._arg {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        bfr.put_u16_le(self._request_id);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u16_le(self._flags);
        match &self._arg {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        serialize_bytes!(bfr, self._info.as_bytes());

        serialize_footer(bfr);
    }
}
