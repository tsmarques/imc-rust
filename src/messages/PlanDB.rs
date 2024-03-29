//###########################################################################
// Licensed under the Apache License, Version 2.0 (the "License");          #
// you may not use this file except in compliance with the License.         #
// You may obtain a copy of the License at                                  #
//                                                                          #
// http://www.apache.org/licenses/LICENSE-2.0                               #
//                                                                          #
// Unless required by applicable law or agreed to in writing, software      #
// distributed under the License is distributed on an "AS IS" BASIS,        #
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. #
// See the License for the specific language governing permissions and      #
// limitations under the License.                                           #
//###########################################################################
// Author: Tiago Sá Marques                                                 #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header;
use crate::Message;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request
    DBT_REQUEST = 0,
    /// Reply -- Success
    DBT_SUCCESS = 1,
    /// Reply -- Failure
    DBT_FAILURE = 2,
    /// Reply -- In Progress
    DBT_IN_PROGRESS = 3,
}

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Set Plan
    DBOP_SET = 0,
    /// Delete Plan
    DBOP_DEL = 1,
    /// Get Plan
    DBOP_GET = 2,
    /// Get Plan Info
    DBOP_GET_INFO = 3,
    /// Clear Database
    DBOP_CLEAR = 4,
    /// Get Database State (Simple)
    DBOP_GET_STATE = 5,
    /// Get Database State (Detailed)
    DBOP_GET_DSTATE = 6,
    /// Boot Notification
    DBOP_BOOT = 7,
}

/// Request/reply to plan database.
#[derive(Default, Clone)]
pub struct PlanDB {
    /// Message Header
    pub _header: Header,
    /// Indicates if the message is a request, or a reply to a
    /// previous request.
    pub _type: u8,
    /// Indicates the operation affecting the DB.
    /// The operation may relate to a single plan or the entire plan DB.
    /// For each request,  a plan DB may reply with any number of 'in progress'
    /// replies followed by a success or a failure reply.
    /// The 'op', 'request_id' and 'plan_id' fields of a request will be echoed
    /// in one or more responses to that request.
    /// The operation at stake also determines a certain type of the 'arg' field,
    /// and whether or not the 'plan_id' field needs to be set.
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
    fn new() -> PlanDB {
        PlanDB {
            _header: Header::new(556),
            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        556
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        556
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts;
        if let Some(m) = &mut self._arg {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._arg {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._arg {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._arg {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._arg {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(556);
        self._type = Default::default();
        self._op = Default::default();
        self._request_id = Default::default();
        self._plan_id = Default::default();
        self._arg = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
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
