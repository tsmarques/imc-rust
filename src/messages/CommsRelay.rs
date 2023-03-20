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

/// In this maneuver, a vehicle drives to the center of two other
/// systems (a, b) in order to be used as a communications relay.
#[derive(Default, Clone)]
pub struct CommsRelay {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude for start point.
    pub _lat: f64,
    /// WGS-84 Longitude for start point.
    pub _lon: f64,
    /// Reference speed.
    pub _speed: f32,
    /// Reference speed units.
    pub _speed_units: u8,
    /// Duration of maneuver, 0 for unlimited duration.
    pub _duration: u16,
    /// The IMC id of the system A.
    pub _sys_a: u16,
    /// The IMC id of the system B.
    pub _sys_b: u16,
    /// Move only if the distance to the target is bigger than this
    /// threshold.
    pub _move_threshold: f32,
}

impl Message for CommsRelay {
    fn new() -> CommsRelay {
        CommsRelay {
            _header: Header::new(472),
            _lat: Default::default(),
            _lon: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _duration: Default::default(),
            _sys_a: Default::default(),
            _sys_b: Default::default(),
            _move_threshold: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        472
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        472
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
        self._header = Header::new(472);
        self._lat = Default::default();
        self._lon = Default::default();
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._duration = Default::default();
        self._sys_a = Default::default();
        self._sys_b = Default::default();
        self._move_threshold = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        31
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_u16_le(self._duration);
        bfr.put_u16_le(self._sys_a);
        bfr.put_u16_le(self._sys_b);
        bfr.put_f32_le(self._move_threshold);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._duration = bfr.get_u16_le();
        self._sys_a = bfr.get_u16_le();
        self._sys_b = bfr.get_u16_le();
        self._move_threshold = bfr.get_f32_le();
        Ok(())
    }
}
