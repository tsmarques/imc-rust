use crate::Message::*;

use crate::DUNE_IMC_CONST_NULL_ID;

use bytes::BufMut;

use crate::Header::Header;

use crate::TrexToken::TrexToken;

use crate::packet::ImcError;
use crate::packet::*;

#[allow(non_camel_case_types)]
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

/// This message is used to control TREX execution
#[derive(Default)]
pub struct TrexOperation {
    /// IMC Header
    pub header: Header,

    pub _op: u8,

    /// The id of the goal, if applicable (OP == POST_GOAL || OP == RECALL_GOAL)
    pub _goal_id: String,

    /// Goal / observation to post, if applicable (OP == POST_GOAL || OP == POST_TOKEN)
    pub _token: Option<TrexToken>,
}

impl Message for TrexOperation {
    fn new() -> Self
    where
        Self: Sized,
    {
        let msg = TrexOperation {
            header: Header::new(655),

            _op: Default::default(),
            _goal_id: Default::default(),
            _token: Default::default(),
        };

        msg
    }

    fn fromHeader(hdr: Header) -> Self
    where
        Self: Sized,
    {
        let msg = TrexOperation {
            header: hdr,

            _op: Default::default(),
            _goal_id: Default::default(),
            _token: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        655
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        655
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self.header
    }

    fn clear(&mut self) {
        self.header.clear();

        self._op = Default::default();
        self._goal_id = Default::default();
        self._token = Default::default();
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;

        dyn_size += self._goal_id.len() + 2;

        inline_message_serialization_size!(dyn_size, self._token);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._goal_id.as_bytes());
        serialize_inline_message!(bfr, self._token);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._goal_id);
        self._token = deserialize_inline_as::<TrexToken>(bfr).ok();

        Ok(())
    }
}
