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

/// This maneuver behaves by following a point.
#[derive(Default, Clone)]
pub struct FollowPoint {
    /// Message Header
    pub _header: Header,
    /// The identifier of the point source to follow (via RemoteSensorInfo or EstimatedState).
    pub _target: String,
    /// Use this speed when travelling from afar.
    pub _max_speed: f32,
    /// Indicates the units used for the maximum speed value.
    pub _speed_units: u8,
    /// WGS-84 Latitude of maneuver in the map. Ignored during execution.
    pub _lat: f64,
    /// WGS-84 Longitude of maneuver in the map. Ignored during execution.
    pub _lon: f64,
    /// Use z_units to specify whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for FollowPoint {
    fn new() -> FollowPoint {
        FollowPoint {
            _header: Header::new(494),
            _target: Default::default(),
            _max_speed: Default::default(),
            _speed_units: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        494
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        494
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
        self._header = Header::new(494);
        self._target = Default::default();
        self._max_speed = Default::default();
        self._speed_units = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        26
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._target.len() + 2;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._target.as_bytes());
        bfr.put_f32_le(self._max_speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._target);
        self._max_speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
