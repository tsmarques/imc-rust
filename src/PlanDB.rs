//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::{Buf, BufMut};

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type.
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request.
    DBT_REQUEST = 0,
    /// Reply -- Success.
    DBT_SUCCESS = 1,
    /// Reply -- Failure.
    DBT_FAILURE = 2,
    /// Reply -- In Progress.
    DBT_IN_PROGRESS = 3,
}

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Set Plan.
    DBOP_SET = 0,
    /// Delete Plan.
    DBOP_DEL = 1,
    /// Get Plan.
    DBOP_GET = 2,
    /// Get Plan Info.
    DBOP_GET_INFO = 3,
    /// Clear Database.
    DBOP_CLEAR = 4,
    /// Get Database State (Simple).
    DBOP_GET_STATE = 5,
    /// Get Database State (Detailed).
    DBOP_GET_DSTATE = 6,
    /// Boot Notification.
    DBOP_BOOT = 7,
}

/// Request/reply to plan database.
#[derive(Default)]
pub struct PlanDB {
    /// Message Header.
    pub _header: Header,
    /// Type.
    pub _type: u8,
    /// Operation.
    pub _op: u8,
    /// Request ID.
    pub _request_id: u16,
    /// Plan ID.
    pub _plan_id: String,
    /// Argument.
    pub _arg: Option<Box<dyn Message>>,
    /// Complementary Information.
    pub _info: String,
}

impl Message for PlanDB {
    fn new() -> PlanDB
    where
        Self: Sized,
    {
        let msg = PlanDB {
            _header: Header::new(556),
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
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        556
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
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
