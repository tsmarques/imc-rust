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

use crate::Header;
use crate::Message;

/// A "Formation Plan" is a maneuver specifying a plan for a team of vehicles.
/// The maneuver defines:
/// - Vehicles included in the formation group
/// - Formation shape configuration
/// - Plan (set of maneuvers) to be followed by the formation center
/// - Speed at which that plan is followed
/// - Path contrains (virtual leader bank limit)
/// - Supervision settings
#[derive(Default, Clone)]
pub struct FormationPlanExecution {
    /// Message Header
    pub _header: Header,
    /// Target group for the formation plan.
    pub _group_name: String,
    /// Name of the formation configuration.
    pub _formation_name: String,
    /// The flight plan's identifier.
    /// Flight plan defined to be tracked by the formation leader.
    pub _plan_id: String,
    /// Verbose text description of plan.
    pub _description: String,
    /// Formation leader flight airspeed during the plan tracking.
    pub _leader_speed: f32,
    /// Formation leader flight bank limit during the plan tracking.
    pub _leader_bank_lim: f32,
    /// Limit for the position mismatch between real and simulated position, before a maneuver abort is asserted.
    pub _pos_sim_err_lim: f32,
    /// Warning threshold for the position mismatch between real and simulated position.
    /// Above this threshold a time-out limit is evaluated to assert a maneuver abort state.
    pub _pos_sim_err_wrn: f32,
    /// The amount of time the maneuver is allowed to run after the position mismatch threshold is surpassed.
    pub _pos_sim_err_timeout: u16,
    /// Threshold for the convergence measure, above which a time-out limit
    /// is evaluated to assert a maneuver abort state.
    pub _converg_max: f32,
    /// The amount of time the maneuver is allowed to run after the convergence threshold is surpassed.
    pub _converg_timeout: u16,
    /// The amount of time the maneuver is allowed to run without any update on the other formation vehicles states.
    pub _comms_timeout: u16,
    /// Turbulence limit above which a maneuver abort is asserted.
    pub _turb_lim: f32,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for FormationPlanExecution {
    fn new() -> FormationPlanExecution {
        FormationPlanExecution {
            _header: Header::new(477),
            _group_name: Default::default(),
            _formation_name: Default::default(),
            _plan_id: Default::default(),
            _description: Default::default(),
            _leader_speed: Default::default(),
            _leader_bank_lim: Default::default(),
            _pos_sim_err_lim: Default::default(),
            _pos_sim_err_wrn: Default::default(),
            _pos_sim_err_timeout: Default::default(),
            _converg_max: Default::default(),
            _converg_timeout: Default::default(),
            _comms_timeout: Default::default(),
            _turb_lim: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        477
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        477
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

    fn clear(&mut self) {
        self._header = Header::new(477);
        self._group_name = Default::default();
        self._formation_name = Default::default();
        self._plan_id = Default::default();
        self._description = Default::default();
        self._leader_speed = Default::default();
        self._leader_bank_lim = Default::default();
        self._pos_sim_err_lim = Default::default();
        self._pos_sim_err_wrn = Default::default();
        self._pos_sim_err_timeout = Default::default();
        self._converg_max = Default::default();
        self._converg_timeout = Default::default();
        self._comms_timeout = Default::default();
        self._turb_lim = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        30
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._group_name.len() + 2;
        dyn_size += self._formation_name.len() + 2;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._description.len() + 2;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._group_name.as_bytes());
        serialize_bytes!(bfr, self._formation_name.as_bytes());
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._description.as_bytes());
        bfr.put_f32_le(self._leader_speed);
        bfr.put_f32_le(self._leader_bank_lim);
        bfr.put_f32_le(self._pos_sim_err_lim);
        bfr.put_f32_le(self._pos_sim_err_wrn);
        bfr.put_u16_le(self._pos_sim_err_timeout);
        bfr.put_f32_le(self._converg_max);
        bfr.put_u16_le(self._converg_timeout);
        bfr.put_u16_le(self._comms_timeout);
        bfr.put_f32_le(self._turb_lim);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._group_name);
        deserialize_string!(bfr, self._formation_name);
        deserialize_string!(bfr, self._plan_id);
        deserialize_string!(bfr, self._description);
        self._leader_speed = bfr.get_f32_le();
        self._leader_bank_lim = bfr.get_f32_le();
        self._pos_sim_err_lim = bfr.get_f32_le();
        self._pos_sim_err_wrn = bfr.get_f32_le();
        self._pos_sim_err_timeout = bfr.get_u16_le();
        self._converg_max = bfr.get_f32_le();
        self._converg_timeout = bfr.get_u16_le();
        self._comms_timeout = bfr.get_u16_le();
        self._turb_lim = bfr.get_f32_le();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
