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
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// State.
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Blocked.
    PCS_BLOCKED = 0,
    /// Ready.
    PCS_READY = 1,
    /// Initializing.
    PCS_INITIALIZING = 2,
    /// Executing.
    PCS_EXECUTING = 3,
}

/// Last Plan Outcome.
#[allow(non_camel_case_types)]
pub enum LastPlanOutcomeEnum {
    /// None.
    LPO_NONE = 0,
    /// Success.
    LPO_SUCCESS = 1,
    /// Failure.
    LPO_FAILURE = 2,
}

/// State of plan control.
#[derive(Default)]
pub struct PlanControlState {
    /// Message Header.
    pub _header: Header,
    /// State.
    pub _state: u8,
    /// Plan -- ID.
    pub _plan_id: String,
    /// Plan -- ETA.
    pub _plan_eta: i32,
    /// Plan -- Progress.
    pub _plan_progress: f32,
    /// Maneuver -- ID.
    pub _man_id: String,
    /// Maneuver -- Type.
    pub _man_type: u16,
    /// Maneuver -- ETA.
    pub _man_eta: i32,
    /// Last Plan Outcome.
    pub _last_outcome: u8,
}

impl Message for PlanControlState {
    fn new() -> PlanControlState {
        let msg = PlanControlState {
            _header: Header::new(560),
            _state: Default::default(),
            _plan_id: Default::default(),
            _plan_eta: Default::default(),
            _plan_progress: Default::default(),
            _man_id: Default::default(),
            _man_type: Default::default(),
            _man_eta: Default::default(),
            _last_outcome: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        560
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        560
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(560);
        self._state = Default::default();
        self._plan_id = Default::default();
        self._plan_eta = Default::default();
        self._plan_progress = Default::default();
        self._man_id = Default::default();
        self._man_type = Default::default();
        self._man_eta = Default::default();
        self._last_outcome = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._man_id.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_i32_le(self._plan_eta);
        bfr.put_f32_le(self._plan_progress);
        serialize_bytes!(bfr, self._man_id.as_bytes());
        bfr.put_u16_le(self._man_type);
        bfr.put_i32_le(self._man_eta);
        bfr.put_u8(self._last_outcome);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        deserialize_string!(bfr, self._plan_id);
        self._plan_eta = bfr.get_i32_le();
        self._plan_progress = bfr.get_f32_le();
        deserialize_string!(bfr, self._man_id);
        self._man_type = bfr.get_u16_le();
        self._man_eta = bfr.get_i32_le();
        self._last_outcome = bfr.get_u8();
        Ok(())
    }
}
