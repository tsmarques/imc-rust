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
use crate::FormationControlParams::FormationControlParams;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request
    FC_REQUEST = 0,
    /// Report
    FC_REPORT = 1,
}

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Start
    OP_START = 0,
    /// Stop
    OP_STOP = 1,
    /// Ready
    OP_READY = 2,
    /// Executing
    OP_EXECUTING = 3,
    /// Failure
    OP_FAILURE = 4,
}

/// Formation control performance evaluation variables.
#[derive(Default, Clone)]
pub struct FormationEvaluation {
    /// Message Header
    pub _header: Header,
    /// Indicates if the message is a request, or a reply to a previous request.
    pub _type: u8,
    /// Operation to perform.
    pub _op: u8,
    /// Mean position error relative to the formation reference.
    pub _err_mean: f32,
    /// Overall minimum distance to any other vehicle in the formation.
    pub _dist_min_abs: f32,
    /// Mean minimum distance to any other vehicle in the formation.
    pub _dist_min_mean: f32,
    /// Mean minimum distance to any other vehicle in the formation.
    pub _roll_rate_mean: f32,
    /// Period over which the evaluation data is averaged.
    pub _time: f32,
    /// Formation controller paramenters during the evaluation period.
    pub _ControlParams: Option<FormationControlParams>,
}

impl Message for FormationEvaluation {
    fn new() -> FormationEvaluation {
        FormationEvaluation {
            _header: Header::new(823),
            _type: Default::default(),
            _op: Default::default(),
            _err_mean: Default::default(),
            _dist_min_abs: Default::default(),
            _dist_min_mean: Default::default(),
            _roll_rate_mean: Default::default(),
            _time: Default::default(),
            _ControlParams: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        823
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        823
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
        if let Some(m) = &mut self._ControlParams {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._ControlParams {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._ControlParams {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._ControlParams {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._ControlParams {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(823);
        self._type = Default::default();
        self._op = Default::default();
        self._err_mean = Default::default();
        self._dist_min_abs = Default::default();
        self._dist_min_mean = Default::default();
        self._roll_rate_mean = Default::default();
        self._time = Default::default();
        self._ControlParams = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        22
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._ControlParams);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        bfr.put_f32_le(self._err_mean);
        bfr.put_f32_le(self._dist_min_abs);
        bfr.put_f32_le(self._dist_min_mean);
        bfr.put_f32_le(self._roll_rate_mean);
        bfr.put_f32_le(self._time);
        serialize_inline_message!(bfr, self._ControlParams);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._op = bfr.get_u8();
        self._err_mean = bfr.get_f32_le();
        self._dist_min_abs = bfr.get_f32_le();
        self._dist_min_mean = bfr.get_f32_le();
        self._roll_rate_mean = bfr.get_f32_le();
        self._time = bfr.get_f32_le();
        self._ControlParams = deserialize_inline_as::<FormationControlParams>(bfr).ok();
        Ok(())
    }
}
