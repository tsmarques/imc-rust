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

use crate::Header::Header;
use crate::Message::*;

/// Action
#[allow(non_camel_case_types)]
pub enum ActionEnum {
    /// Request
    OP_REQ = 0,
    /// Set
    OP_SET = 1,
    /// Report
    OP_REP = 2,
}

/// Formation controller paramenters, as: trajectory gains,
/// control boundary layer thickness, and formation shape gains.
#[derive(Default, Clone)]
pub struct FormationControlParams {
    /// Message Header
    pub _header: Header,
    /// Action on the vehicle formation control parameters.
    pub _Action: u8,
    /// Trajectory gain over the vehicle longitudinal direction.
    pub _lon_gain: f32,
    /// Trajectory gain over the vehicle lateral direction.
    pub _lat_gain: f32,
    /// Control sliding surface boundary layer thickness.
    pub _bond_thick: f32,
    /// Formation shape gain (absolute vehicle position tracking).
    /// Leader control importance gain (relative to the sum of every other formation vehicle).
    pub _lead_gain: f32,
    /// Collision avoidance and formation shape gain (position tracking relative to the other formation vehicles).
    /// Individual vehicle importance gain (relative to the leader), when the relative position or the velocity state indicate higher probability of collision.
    pub _deconfl_gain: f32,
    /// Switch gain to compensate the worst case of the wind flow acceleration.
    pub _accel_switch_gain: f32,
    /// Inter-vehicle safety distance.
    pub _safe_dist: f32,
    /// Distance offset which defines the buffer area beyond the safety distace.
    pub _deconflict_offset: f32,
    /// Safety margin to compensate for possible shortfalls from the predicted maximum acceleration that a vehicle can generate.
    pub _accel_safe_margin: f32,
    /// Maximum predicted longitudinal acceleration a vehicle can generate.
    pub _accel_lim_x: f32,
}

impl Message for FormationControlParams {
    fn new() -> FormationControlParams {
        

        FormationControlParams {
            _header: Header::new(822),
            _Action: Default::default(),
            _lon_gain: Default::default(),
            _lat_gain: Default::default(),
            _bond_thick: Default::default(),
            _lead_gain: Default::default(),
            _deconfl_gain: Default::default(),
            _accel_switch_gain: Default::default(),
            _safe_dist: Default::default(),
            _deconflict_offset: Default::default(),
            _accel_safe_margin: Default::default(),
            _accel_lim_x: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        822
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        822
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
        self._header = Header::new(822);
        self._Action = Default::default();
        self._lon_gain = Default::default();
        self._lat_gain = Default::default();
        self._bond_thick = Default::default();
        self._lead_gain = Default::default();
        self._deconfl_gain = Default::default();
        self._accel_switch_gain = Default::default();
        self._safe_dist = Default::default();
        self._deconflict_offset = Default::default();
        self._accel_safe_margin = Default::default();
        self._accel_lim_x = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        41
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._Action);
        bfr.put_f32_le(self._lon_gain);
        bfr.put_f32_le(self._lat_gain);
        bfr.put_f32_le(self._bond_thick);
        bfr.put_f32_le(self._lead_gain);
        bfr.put_f32_le(self._deconfl_gain);
        bfr.put_f32_le(self._accel_switch_gain);
        bfr.put_f32_le(self._safe_dist);
        bfr.put_f32_le(self._deconflict_offset);
        bfr.put_f32_le(self._accel_safe_margin);
        bfr.put_f32_le(self._accel_lim_x);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._Action = bfr.get_u8();
        self._lon_gain = bfr.get_f32_le();
        self._lat_gain = bfr.get_f32_le();
        self._bond_thick = bfr.get_f32_le();
        self._lead_gain = bfr.get_f32_le();
        self._deconfl_gain = bfr.get_f32_le();
        self._accel_switch_gain = bfr.get_f32_le();
        self._safe_dist = bfr.get_f32_le();
        self._deconflict_offset = bfr.get_f32_le();
        self._accel_safe_margin = bfr.get_f32_le();
        self._accel_lim_x = bfr.get_f32_le();
        Ok(())
    }
}
