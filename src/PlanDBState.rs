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
// Author: Tiago Sá Marques                                                 #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
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
#[derive(Default)]
pub struct PlanDBState {
    /// Message Header.
    pub _header: Header,
    /// Plan -- Count.
    pub _plan_count: u16,
    /// Plan -- Size of all plans.
    pub _plan_size: u32,
    /// Last Change -- Time.
    pub _change_time: f64,
    /// Last Change -- Source Address.
    pub _change_sid: u16,
    /// Last Change -- Source Name.
    pub _change_sname: String,
    /// MD5.
    pub _md5: Vec<u8>,
    /// Plan info.
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
