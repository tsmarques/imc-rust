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

/// This messages is used to record system activity parameters. These
/// parameters are mainly used for used for maintenance purposes.
#[derive(Default, Clone)]
pub struct Tachograph {
    /// Message Header
    pub _header: Header,
    /// The time when the last service was performed. The number of
    /// seconds is represented in Universal Coordinated Time (UCT) in
    /// seconds since Jan 1, 1970.
    pub _timestamp_last_service: f64,
    /// Amount of time until the next recommended service.
    pub _time_next_service: f32,
    /// Amount of time the motor can run until the next recommended service.
    pub _time_motor_next_service: f32,
    /// Amount of time the system spent idle on the ground.
    pub _time_idle_ground: f32,
    /// Amount of time the system spent idle in the air.
    pub _time_idle_air: f32,
    /// Amount of time the system spent idle on the water (not submerged).
    pub _time_idle_water: f32,
    /// Amount of time the system spent idle underwater.
    pub _time_idle_underwater: f32,
    /// Amount of time the system spent idle in an unknown medium.
    pub _time_idle_unknown: f32,
    /// Amount of time the system spent on the ground with the motor running.
    pub _time_motor_ground: f32,
    /// Amount of time the system spent in the air with the motor running.
    pub _time_motor_air: f32,
    /// Amount of time the system spent on the water (not submerged) with the motor running.
    pub _time_motor_water: f32,
    /// Amount of time the system spent underwater with the motor running.
    pub _time_motor_underwater: f32,
    /// Amount of time the system spent in an unknown medium with the motor running.
    pub _time_motor_unknown: f32,
    /// The minimum recorded RPM value.
    pub _rpm_min: i16,
    /// The maximum recorded RPM value.
    pub _rpm_max: i16,
    /// The maximum recorded depth value.
    pub _depth_max: f32,
}

impl Message for Tachograph {
    fn new() -> Tachograph {
        Tachograph {
            _header: Header::new(905),
            _timestamp_last_service: Default::default(),
            _time_next_service: Default::default(),
            _time_motor_next_service: Default::default(),
            _time_idle_ground: Default::default(),
            _time_idle_air: Default::default(),
            _time_idle_water: Default::default(),
            _time_idle_underwater: Default::default(),
            _time_idle_unknown: Default::default(),
            _time_motor_ground: Default::default(),
            _time_motor_air: Default::default(),
            _time_motor_water: Default::default(),
            _time_motor_underwater: Default::default(),
            _time_motor_unknown: Default::default(),
            _rpm_min: Default::default(),
            _rpm_max: Default::default(),
            _depth_max: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        905
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        905
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
        self._header = Header::new(905);
        self._timestamp_last_service = Default::default();
        self._time_next_service = Default::default();
        self._time_motor_next_service = Default::default();
        self._time_idle_ground = Default::default();
        self._time_idle_air = Default::default();
        self._time_idle_water = Default::default();
        self._time_idle_underwater = Default::default();
        self._time_idle_unknown = Default::default();
        self._time_motor_ground = Default::default();
        self._time_motor_air = Default::default();
        self._time_motor_water = Default::default();
        self._time_motor_underwater = Default::default();
        self._time_motor_unknown = Default::default();
        self._rpm_min = Default::default();
        self._rpm_max = Default::default();
        self._depth_max = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        64
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._timestamp_last_service);
        bfr.put_f32_le(self._time_next_service);
        bfr.put_f32_le(self._time_motor_next_service);
        bfr.put_f32_le(self._time_idle_ground);
        bfr.put_f32_le(self._time_idle_air);
        bfr.put_f32_le(self._time_idle_water);
        bfr.put_f32_le(self._time_idle_underwater);
        bfr.put_f32_le(self._time_idle_unknown);
        bfr.put_f32_le(self._time_motor_ground);
        bfr.put_f32_le(self._time_motor_air);
        bfr.put_f32_le(self._time_motor_water);
        bfr.put_f32_le(self._time_motor_underwater);
        bfr.put_f32_le(self._time_motor_unknown);
        bfr.put_i16_le(self._rpm_min);
        bfr.put_i16_le(self._rpm_max);
        bfr.put_f32_le(self._depth_max);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timestamp_last_service = bfr.get_f64_le();
        self._time_next_service = bfr.get_f32_le();
        self._time_motor_next_service = bfr.get_f32_le();
        self._time_idle_ground = bfr.get_f32_le();
        self._time_idle_air = bfr.get_f32_le();
        self._time_idle_water = bfr.get_f32_le();
        self._time_idle_underwater = bfr.get_f32_le();
        self._time_idle_unknown = bfr.get_f32_le();
        self._time_motor_ground = bfr.get_f32_le();
        self._time_motor_air = bfr.get_f32_le();
        self._time_motor_water = bfr.get_f32_le();
        self._time_motor_underwater = bfr.get_f32_le();
        self._time_motor_unknown = bfr.get_f32_le();
        self._rpm_min = bfr.get_i16_le();
        self._rpm_max = bfr.get_i16_le();
        self._depth_max = bfr.get_f32_le();
        Ok(())
    }
}
