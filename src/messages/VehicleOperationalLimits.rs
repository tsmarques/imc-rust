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

/// Action on the vehicle operational limits
#[allow(non_camel_case_types)]
pub enum ActiononthevehicleoperationallimitsEnum {
    /// Request
    OP_REQUEST = 0,
    /// Set
    OP_SET = 1,
    /// Report
    OP_REPORT = 2,
}

/// Vehicle opertional limits.
/// For aircraft this should represent the flight envelope and the dynamic contraints.
#[derive(Default, Clone)]
pub struct VehicleOperationalLimits {
    /// Message Header
    pub _header: Header,
    /// Action on the vehicle operation limits
    pub _op: u8,
    /// Minimum operation speed.
    /// For aircraft this is equal or larger then the stall speed.
    pub _speed_min: f32,
    /// Maximum operation speed.
    /// For aircraft this is limited by the engine power or structural contrains.
    pub _speed_max: f32,
    /// Maximum longitudinal acceleration.
    pub _long_accel: f32,
    /// Maximum altitude above mean-sea-level.
    pub _alt_max_msl: f32,
    /// Maximum dive rate (negative vertical speed) as a fraction of the longitudinal speed.
    pub _dive_fraction_max: f32,
    /// Maximum climb rate (positive vertical speed) as a fraction of the longitudinal speed.
    pub _climb_fraction_max: f32,
    /// Limit to the bank angle (roll; angle over the xx body-axis).
    pub _bank_max: f32,
    /// Limit to the bank angular rate (roll; angle over the xx body-axis).
    pub _p_max: f32,
    /// Minimum pitch angle (angle over the xx body-axis).
    pub _pitch_min: f32,
    /// Maximum pitch angle (angle over the xx body-axis).
    pub _pitch_max: f32,
    /// Maximum pitch angular rate (angle over the xx body-axis).
    pub _q_max: f32,
    /// Minimum load factor, i.e., maximum positive acceleration in the zz body-axis
    /// as a factor of the gravity acceleration at mean-sea-level.
    pub _g_min: f32,
    /// Maximum load factor, i.e., maximum negative acceleration in the zz body-axis
    /// as a factor of the gravity acceleration at mean-sea-level.
    pub _g_max: f32,
    /// Maximum lateral load factor, i.e., maximum acceleration in the yy body-axis
    /// as a factor of the gravity acceleration at mean-sea-level.
    pub _g_lat_max: f32,
    /// Minimum motor RPMs.
    pub _rpm_min: f32,
    /// Maximum motor RPMs.
    pub _rpm_max: f32,
    /// Maximum motor RPMs' rate of change.
    pub _rpm_rate_max: f32,
}

impl Message for VehicleOperationalLimits {
    fn new() -> VehicleOperationalLimits {
        VehicleOperationalLimits {
            _header: Header::new(16),
            _op: Default::default(),
            _speed_min: Default::default(),
            _speed_max: Default::default(),
            _long_accel: Default::default(),
            _alt_max_msl: Default::default(),
            _dive_fraction_max: Default::default(),
            _climb_fraction_max: Default::default(),
            _bank_max: Default::default(),
            _p_max: Default::default(),
            _pitch_min: Default::default(),
            _pitch_max: Default::default(),
            _q_max: Default::default(),
            _g_min: Default::default(),
            _g_max: Default::default(),
            _g_lat_max: Default::default(),
            _rpm_min: Default::default(),
            _rpm_max: Default::default(),
            _rpm_rate_max: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        16
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        16
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
        self._header = Header::new(16);
        self._op = Default::default();
        self._speed_min = Default::default();
        self._speed_max = Default::default();
        self._long_accel = Default::default();
        self._alt_max_msl = Default::default();
        self._dive_fraction_max = Default::default();
        self._climb_fraction_max = Default::default();
        self._bank_max = Default::default();
        self._p_max = Default::default();
        self._pitch_min = Default::default();
        self._pitch_max = Default::default();
        self._q_max = Default::default();
        self._g_min = Default::default();
        self._g_max = Default::default();
        self._g_lat_max = Default::default();
        self._rpm_min = Default::default();
        self._rpm_max = Default::default();
        self._rpm_rate_max = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        69
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_f32_le(self._speed_min);
        bfr.put_f32_le(self._speed_max);
        bfr.put_f32_le(self._long_accel);
        bfr.put_f32_le(self._alt_max_msl);
        bfr.put_f32_le(self._dive_fraction_max);
        bfr.put_f32_le(self._climb_fraction_max);
        bfr.put_f32_le(self._bank_max);
        bfr.put_f32_le(self._p_max);
        bfr.put_f32_le(self._pitch_min);
        bfr.put_f32_le(self._pitch_max);
        bfr.put_f32_le(self._q_max);
        bfr.put_f32_le(self._g_min);
        bfr.put_f32_le(self._g_max);
        bfr.put_f32_le(self._g_lat_max);
        bfr.put_f32_le(self._rpm_min);
        bfr.put_f32_le(self._rpm_max);
        bfr.put_f32_le(self._rpm_rate_max);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._speed_min = bfr.get_f32_le();
        self._speed_max = bfr.get_f32_le();
        self._long_accel = bfr.get_f32_le();
        self._alt_max_msl = bfr.get_f32_le();
        self._dive_fraction_max = bfr.get_f32_le();
        self._climb_fraction_max = bfr.get_f32_le();
        self._bank_max = bfr.get_f32_le();
        self._p_max = bfr.get_f32_le();
        self._pitch_min = bfr.get_f32_le();
        self._pitch_max = bfr.get_f32_le();
        self._q_max = bfr.get_f32_le();
        self._g_min = bfr.get_f32_le();
        self._g_max = bfr.get_f32_le();
        self._g_lat_max = bfr.get_f32_le();
        self._rpm_min = bfr.get_f32_le();
        self._rpm_max = bfr.get_f32_le();
        self._rpm_rate_max = bfr.get_f32_le();
        Ok(())
    }
}
