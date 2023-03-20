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

/// The Station Keeping maneuver makes the vehicle come to the surface
/// and then enter a given circular perimeter around a waypoint coordinate
/// for a certain amount of time.
#[derive(Default, Clone)]
pub struct StationKeeping {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude.
    pub _lat: f64,
    /// WGS-84 Longitude.
    pub _lon: f64,
    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
    /// Radius.
    pub _radius: f32,
    /// Duration (0 for unlimited).
    pub _duration: u16,
    /// The value of the desired speed, in the scale specified
    /// by the "Speed Units" field.
    pub _speed: f32,
    /// Indicates the units used for the speed value.
    pub _speed_units: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for StationKeeping {
    fn new() -> StationKeeping {
        StationKeeping {
            _header: Header::new(461),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _radius: Default::default(),
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        461
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        461
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
        self._header = Header::new(461);
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._radius = Default::default();
        self._duration = Default::default();
        self._speed = Default::default();
        self._speed_units = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        32
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
        bfr.put_f32_le(self._radius);
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._radius = bfr.get_f32_le();
        self._duration = bfr.get_u16_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
