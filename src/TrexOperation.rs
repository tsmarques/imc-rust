use crate::Message::*;
use crate::{DUNE_IMC_CONST_SYNC, IMC_CONST_UNK_EID};

use bytes::BufMut;

use crate::Header::Header;

use crate::TrexToken::TrexToken;

pub enum OperationEnum {
    // Post Token
    OP_POST_TOKEN = 1,
    // Post Goal
    OP_POST_GOAL = 2,
    // Recall Goal
    OP_RECALL_GOAL = 3,
    // Request current plan
    OP_REQUEST_PLAN = 4,
    // Report current plan
    OP_REPORT_PLAN = 5,
}

impl OperationEnum {
    pub fn as_primitive(&self) -> u32 {
        match self {
            OP_POST_TOKEN => 1,
            OP_POST_GOAL => 2,
            OP_RECALL_GOAL => 3,
            OP_REQUEST_PLAN => 4,
            OP_REPORT_PLAN => 5,
        }
    }
}

/// This message is used to control TREX execution
pub struct TrexOperation {
    /// IMC Header
    pub header: Header,

    pub _op: u8,

    /// The id of the goal, if applicable (OP == POST_GOAL || OP == RECALL_GOAL)
    pub _goal_id: String,

    /// Goal / observation to post, if applicable (OP == POST_GOAL || OP == POST_TOKEN)
    pub _token: Option<Box<TrexToken>>,
}

impl TrexOperation {
    pub fn new() -> TrexOperation {
        let mut msg = TrexOperation {
            header: Header::new(655),

            _op: Default::default(),
            _goal_id: Default::default(),
            _token: Default::default(),
        };

        msg.set_size(msg.payload_serialization_size() as u16);

        msg
    }
}

impl Message for TrexOperation {
    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn static_id(&self) -> u16 {
        655
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();

        self._goal_id = Default::default();

        match &mut self._token {
            Some(field) => field.clear(),

            None => {}
        }
    }

    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._goal_id.len() + 2;

        match &self._token {
            None => {}
            Some(msg) => {
                dyn_size += msg.dynamic_serialization_size();
            }
        }

        dyn_size
    }

    fn serialize(&self, bfr: &mut bytes::BytesMut) {
        self.header.serialize(bfr);

        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._goal_id.as_bytes());
        match &self._token {
            Some(field) => field.serialize(bfr),

            None => {}
        };

        serialize_footer(bfr);
    }
}
