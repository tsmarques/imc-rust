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

/// System-following maneuver.
#[derive(Default, Clone)]
pub struct FollowSystem {
    /// Message Header
    pub _header: Header,
    /// IMC address of system to follow.
    pub _system: u16,
    /// Duration of maneuver, 0 for unlimited duration.
    pub _duration: u16,
    /// Reference speed.
    pub _speed: f32,
    /// Reference speed units.
    pub _speed_units: u8,
    /// Along-track offset.
    pub _x: f32,
    /// Cross-track offset.
    pub _y: f32,
    /// Coordinate z during follow maneuver. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
}

impl Message for FollowSystem {
    fn new() -> FollowSystem {
        

        FollowSystem {
            _header: Header::new(471),
            _system: Default::default(),
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        471
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        471
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
        self._header = Header::new(471);
        self._system = Default::default();
        self._duration = Default::default();
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        22
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._system);
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._system = bfr.get_u16_le();
        self._duration = bfr.get_u16_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        Ok(())
    }
}
