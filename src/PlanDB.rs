use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
pub enum TypeEnum {
    // Request
    DBT_REQUEST = 0,
    // Reply -- Success
    DBT_SUCCESS = 1,
    // Reply -- Failure
    DBT_FAILURE = 2,
    // Reply -- In Progress
    DBT_IN_PROGRESS = 3,
}

#[allow(non_camel_case_types)]
pub enum OperationEnum {
    // Set Plan
    DBOP_SET = 0,
    // Delete Plan
    DBOP_DEL = 1,
    // Get Plan
    DBOP_GET = 2,
    // Get Plan Info
    DBOP_GET_INFO = 3,
    // Clear Database
    DBOP_CLEAR = 4,
    // Get Database State (Simple)
    DBOP_GET_STATE = 5,
    // Get Database State (Detailed)
    DBOP_GET_DSTATE = 6,
    // Boot Notification
    DBOP_BOOT = 7,
}

#[allow(non_camel_case_types)]
pub mod Type {
    // Request
    pub const DBT_REQUEST: u32 = 0;
    // Reply -- Success
    pub const DBT_SUCCESS: u32 = 1;
    // Reply -- Failure
    pub const DBT_FAILURE: u32 = 2;
    // Reply -- In Progress
    pub const DBT_IN_PROGRESS: u32 = 3;
}

#[allow(non_camel_case_types)]
pub mod Operation {
    // Set Plan
    pub const DBOP_SET: u32 = 0;
    // Delete Plan
    pub const DBOP_DEL: u32 = 1;
    // Get Plan
    pub const DBOP_GET: u32 = 2;
    // Get Plan Info
    pub const DBOP_GET_INFO: u32 = 3;
    // Clear Database
    pub const DBOP_CLEAR: u32 = 4;
    // Get Database State (Simple)
    pub const DBOP_GET_STATE: u32 = 5;
    // Get Database State (Detailed)
    pub const DBOP_GET_DSTATE: u32 = 6;
    // Boot Notification
    pub const DBOP_BOOT: u32 = 7;
}

/// Get detailed state of the entire DB. Successful replies
/// will yield a 'PlanDBState' message in the 'arg' field with
/// individual plan information (in the 'plans_info' field of
/// 'PlanDBState').
#[derive(Default)]
pub struct PlanDB {
    /// IMC Header
    pub header: Header,

    /// Indicates if the message is a request, or a reply to a
    /// previous request.
    pub _type: u8,

    /// PlanDB replies of this type are sent automatically during
    /// bootstrap.
    pub _op: u8,

    /// Request ID. This may be used by interfacing modules,
    /// e.g. using sequence counters, to annotate requests and
    /// appropriately identify replies
    pub _request_id: u16,

    /// Plan identifier for the operation, if one is required.
    pub _plan_id: String,

    /// Request or reply argument.
    pub _arg: Option<Box<dyn Message>>,

    /// Human-readable complementary information. For example this
    /// may be used to detail reasons for failure, or to report
    /// in-progress information.
    pub _info: String,
}

impl Message for PlanDB {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = PlanDB {
            header: Header::new(556),

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = PlanDB {
            header: hdr,

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
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
        556
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        556
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
        self._arg = Default::default();
        self._info = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
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
        serialize_inline_message!(bfr, self._arg);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._op = bfr.get_u8();
        self._request_id = bfr.get_u16_le();
        deserialize_string!(bfr, self._plan_id);
        self._arg = deserialize_inline(bfr).ok();
        deserialize_string!(bfr, self._info);

        Ok(())
    }
}
