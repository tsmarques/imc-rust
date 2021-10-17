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

/// Action on the vehicle operational limits.
#[allow(non_camel_case_types)]
pub enum ActiononthevehicleoperationallimitsEnum {
    /// Request.
    OP_REQUEST = 0,
    /// Set.
    OP_SET = 1,
    /// Report.
    OP_REPORT = 2,
}

/// Vehicle opertional limits.
/// For aircraft this should represent the flight envelope and the dynamic contraints.
#[derive(Default)]
pub struct VehicleOperationalLimits {
    /// Message Header.
    pub _header: Header,
    /// Action on the vehicle operational limits.
    pub _op: u8,
    /// Minimum speed.
    pub _speed_min: f32,
    /// Maximum speed.
    pub _speed_max: f32,
    /// Longitudinal maximum acceleration.
    pub _long_accel: f32,
    /// Maximum MSL altitude.
    pub _alt_max_msl: f32,
    /// Maximum Dive Rate Speed Fraction.
    pub _dive_fraction_max: f32,
    /// Maximum Climb Rate Speed Fraction.
    pub _climb_fraction_max: f32,
    /// Bank limit.
    pub _bank_max: f32,
    /// Bank rate limit.
    pub _p_max: f32,
    /// Minimum pitch angle.
    pub _pitch_min: f32,
    /// Maximum pitch angle.
    pub _pitch_max: f32,
    /// Maximum pitch rate.
    pub _q_max: f32,
    /// Minimum load factor.
    pub _g_min: f32,
    /// Maximum load factor.
    pub _g_max: f32,
    /// Maximum lateral load factor.
    pub _g_lat_max: f32,
    /// Minimum RPMs.
    pub _rpm_min: f32,
    /// Maximum RPMs.
    pub _rpm_max: f32,
    /// Maximum RPM rate.
    pub _rpm_rate_max: f32,
}

impl Message for VehicleOperationalLimits {
    fn new() -> VehicleOperationalLimits
    where
        Self: Sized,
    {
        let msg = VehicleOperationalLimits {
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
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        16
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        16
    }

    fn get_header(&mut self) -> &mut Header {
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
