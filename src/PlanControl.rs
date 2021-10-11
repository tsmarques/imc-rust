use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            PC_REQUEST => 0,
            PC_SUCCESS => 1,
            PC_FAILURE => 2,
            PC_IN_PROGRESS => 3,
        }
    }
}

#[allow(non_camel_case_types)]
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
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u8 {
        match self {
            PC_START => 0,
            PC_STOP => 1,
            PC_LOAD => 2,
            PC_GET => 3,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum FlagsEnum {
    // Calibrate Vehicle
    FLG_CALIBRATE = 0x0001,
    // Ignore Errors
    FLG_IGNORE_ERRORS = 0x0002,
}

impl FlagsEnum {
    /// Match an enum value to its primitive type
    pub fn value(&self) -> u16 {
        match self {
            FLG_CALIBRATE => 0x0001,
            FLG_IGNORE_ERRORS => 0x0002,
        }
    }
}

/// Execute current plan while ignoring some errors that might be active.
#[derive(Default)]
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

impl Message for PlanControl {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanControl {
            header: Header::new(559),

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _flags: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanControl {
            header: hdr,

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _flags: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        559
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        559
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();
        self._op = Default::default();
        self._request_id = Default::default();
        self._plan_id = Default::default();
        self._flags = Default::default();
        self._arg = Default::default();
        self._info = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._plan_id.len() + 2;

        inline_message_serialization_size!(dyn_size, self._arg);

        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        bfr.put_u16_le(self._request_id);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u16_le(self._flags);
        serialize_inline_message!(bfr, self._arg);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._op = bfr.get_u8();
        self._request_id = bfr.get_u16_le();
        deserialize_string!(bfr, self._plan_id);
        self._flags = bfr.get_u16_le();
        self._arg = deserialize_inline(bfr).ok();
        deserialize_string!(bfr, self._info);

        Ok(())
    }
}
