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

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::PlanDBInformation::PlanDBInformation;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Characterizes the state of the entire plan database.
#[derive(Default, Clone)]
pub struct PlanDBState {
    /// Message Header
    pub _header: Header,
    /// Number of stored plans.
    pub _plan_count: u16,
    /// Size of all plans.The value equals the sum of the IMC payload
    /// sizes for 'PlanSpecification' stored in the DB.
    pub _plan_size: u32,
    /// Time of last change (Epoch time).
    pub _change_time: f64,
    /// IMC address for source of last DB change.
    pub _change_sid: u16,
    /// IMC node name for source of last DB change.
    pub _change_sname: String,
    /// MD5 database verification code. The MD5 hash sum is computed
    /// over the stream formed by the MD5 of all plans, ordered by
    /// plan id, in compliance with RFC 1321.
    pub _md5: Vec<u8>,
    /// Individual information for plans.
    pub _plans_info: MessageList<PlanDBInformation>,
}

impl Message for PlanDBState {
    fn new() -> PlanDBState {
        let msg = PlanDBState {
            _header: Header::new(557),
            _plan_count: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
            _plans_info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        557
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        557
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._plans_info {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._plans_info {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._plans_info {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._plans_info {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._plans_info {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(557);
        self._plan_count = Default::default();
        self._plan_size = Default::default();
        self._change_time = Default::default();
        self._change_sid = Default::default();
        self._change_sname = Default::default();
        self._md5 = Default::default();
        self._plans_info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._change_sname.len() + 2;
        dyn_size += self._md5.len() + 2;
        message_list_serialization_size!(dyn_size, self._plans_info);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._plan_count);
        bfr.put_u32_le(self._plan_size);
        bfr.put_f64_le(self._change_time);
        bfr.put_u16_le(self._change_sid);
        serialize_bytes!(bfr, self._change_sname.as_bytes());
        serialize_bytes!(bfr, self._md5.as_slice());
        serialize_message_list!(bfr, self._plans_info);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._plan_count = bfr.get_u16_le();
        self._plan_size = bfr.get_u32_le();
        self._change_time = bfr.get_f64_le();
        self._change_sid = bfr.get_u16_le();
        deserialize_string!(bfr, self._change_sname);
        deserialize_bytes!(bfr, self._md5);
        self._plans_info = deserialize_message_list_as::<PlanDBInformation>(bfr)?;
        Ok(())
    }
}
