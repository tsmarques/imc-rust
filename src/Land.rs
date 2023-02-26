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

/// Automatic landing on the ground, for UAVs.
/// This maneuver specifies the target touchdown location and sets the final approach based on the maneuver bearing and glide slope parameters.
#[derive(Default, Clone)]
pub struct Land {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude of touchdown waypoint.
    pub _lat: f64,
    /// WGS-84 Longitude of touchdown waypoint.
    pub _lon: f64,
    /// Target altitude or height for the automatic landing.
    pub _z: f32,
    /// Units of the z reference and abort z reference.
    pub _z_units: u8,
    /// Maneuver speed reference.
    pub _speed: f32,
    /// Speed units.
    pub _speed_units: u8,
    /// Abort altitude or height. If landing is aborted while executing, the UAV will maintain its course and attempt to climb to the abort z reference.
    pub _abort_z: f32,
    /// Land bearing angle.
    pub _bearing: f64,
    /// Ratio of the distance from the last waypoint to the landing point (touchdown) and the height difference between them.
    pub _glide_slope: u8,
    /// Height difference between the last waypoint to the landing point (touchdown).
    pub _glide_slope_alt: f32,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Land {
    fn new() -> Land {
        

        Land {
            _header: Header::new(492),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _abort_z: Default::default(),
            _bearing: Default::default(),
            _glide_slope: Default::default(),
            _glide_slope_alt: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        492
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        492
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
        self._header = Header::new(492);
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._abort_z = Default::default();
        self._bearing = Default::default();
        self._glide_slope = Default::default();
        self._glide_slope_alt = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        43
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f32_le(self._abort_z);
        bfr.put_f64_le(self._bearing);
        bfr.put_u8(self._glide_slope);
        bfr.put_f32_le(self._glide_slope_alt);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._abort_z = bfr.get_f32_le();
        self._bearing = bfr.get_f64_le();
        self._glide_slope = bfr.get_u8();
        self._glide_slope_alt = bfr.get_f32_le();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
