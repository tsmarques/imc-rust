use crate::imc::Message::*;
use crate::imc::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::imc::Header::Header;

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

impl TypeEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            DBT_REQUEST => 0,
            DBT_SUCCESS => 1,
            DBT_FAILURE => 2,
            DBT_IN_PROGRESS => 3,
        }
    }
}

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

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            DBOP_SET => 0,
            DBOP_DEL => 1,
            DBOP_GET => 2,
            DBOP_GET_INFO => 3,
            DBOP_CLEAR => 4,
            DBOP_GET_STATE => 5,
            DBOP_GET_DSTATE => 6,
            DBOP_BOOT => 7,
        }
    }
}

/// Get detailed state of the entire DB. Successful replies
/// will yield a 'PlanDBState' message in the 'arg' field with
/// individual plan information (in the 'plans_info' field of
/// 'PlanDBState').
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

impl PlanDB {
    pub fn new() -> PlanDB {
        let mut msg = PlanDB {
            header: Header::new(556),

            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for PlanDB {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        556
    }

    fn clear(&mut self) {
        self.header.clear();

        self._type = Default::default();

        self._op = Default::default();

        self._request_id = Default::default();

        self._plan_id = Default::default();

        match &mut self._arg {
            Some(field) => field.clear(),

            None => {}
        }

        self._info = Default::default();
    }

    fn fixed_serialization_size(&self) -> usize {
        4
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
        match &self._arg {
            Some(field) => field.serialize(bfr),

            None => {}
        };
        serialize_bytes!(bfr, self._info.as_bytes());

        serialize_footer(bfr);
    }
}
