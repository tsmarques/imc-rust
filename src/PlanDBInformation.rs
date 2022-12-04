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

use crate::Header::Header;
use crate::Message::*;

#[derive(Default, Clone)]
pub struct PlanDBInformation {
    /// Message Header
    pub _header: Header,
    /// Plan identifier.
    pub _plan_id: String,
    /// Plan size. The value equals the IMC message payload of the
    /// associated 'PlanSpecification' message in bytes.
    pub _plan_size: u16,
    /// Time of last change to the plan (Epoch time).
    pub _change_time: f64,
    /// IMC address for source of last change to the plan.
    pub _change_sid: u16,
    /// IMC node name for source of last change to the plan.
    pub _change_sname: String,
    /// MD5 plan verification code. The value is calculated over the
    /// message payload of the 'PlanSpecification', in compliance with
    /// RFC 1321.
    pub _md5: Vec<u8>,
}

impl Message for PlanDBInformation {
    fn new() -> PlanDBInformation {
        

        PlanDBInformation {
            _header: Header::new(558),
            _plan_id: Default::default(),
            _plan_size: Default::default(),
            _change_time: Default::default(),
            _change_sid: Default::default(),
            _change_sname: Default::default(),
            _md5: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        558
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        558
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

    fn clear(&mut self) {
        self._header = Header::new(558);
        self._plan_id = Default::default();
        self._plan_size = Default::default();
        self._change_time = Default::default();
        self._change_sid = Default::default();
        self._change_sname = Default::default();
        self._md5 = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        12
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._change_sname.len() + 2;
        dyn_size += self._md5.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u16_le(self._plan_size);
        bfr.put_f64_le(self._change_time);
        bfr.put_u16_le(self._change_sid);
        serialize_bytes!(bfr, self._change_sname.as_bytes());
        serialize_bytes!(bfr, self._md5.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._plan_id);
        self._plan_size = bfr.get_u16_le();
        self._change_time = bfr.get_f64_le();
        self._change_sid = bfr.get_u16_le();
        deserialize_string!(bfr, self._change_sname);
        deserialize_bytes!(bfr, self._md5);
        Ok(())
    }
}
