//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header;
use crate::Message;
use crate::PlanDB::PlanDB;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Request Type
#[allow(non_camel_case_types)]
pub enum RequestTypeEnum {
    /// Request
    VPS_REQ = 0,
    /// Reply -- Valid
    VPS_VALID = 1,
    /// Reply -- Invalid
    VPS_INVALID = 2,
}

/// Use to validate plans
#[derive(Default, Clone)]
pub struct ValidatePlan {
    /// Message Header
    pub _header: Header,
    /// Type of request.
    pub _type: u8,
    /// An inline plan specification to be used both in requests and replies.
    pub _plan: Option<PlanDB>,
}

impl Message for ValidatePlan {
    fn new() -> ValidatePlan {
        

        ValidatePlan {
            _header: Header::new(2007),
            _type: Default::default(),
            _plan: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2007
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2007
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._plan {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._plan {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._plan {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._plan {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._plan {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(2007);
        self._type = Default::default();
        self._plan = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._plan);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        serialize_inline_message!(bfr, self._plan);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._plan = deserialize_inline_as::<PlanDB>(bfr).ok();
        Ok(())
    }
}
