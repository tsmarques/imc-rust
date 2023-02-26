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

/// Message containing the x, y and z coordinates of object in the real world.
#[derive(Default, Clone)]
pub struct GetWorldCoordinates {
    /// Message Header
    pub _header: Header,
    /// True when system is tracking.
    pub _tracking: u8,
    /// Latitude of the real world frame origin.
    pub _lat: f64,
    /// Longitude of the real world frame origin.
    pub _lon: f64,
    /// X offsets of the target in the real world frame.
    pub _x: f32,
    /// Y offsets of the target in the real world frame.
    pub _y: f32,
    /// Z offsets of the target in the real world frame.
    pub _z: f32,
}

impl Message for GetWorldCoordinates {
    fn new() -> GetWorldCoordinates {
        

        GetWorldCoordinates {
            _header: Header::new(897),
            _tracking: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        897
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        897
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
        self._header = Header::new(897);
        self._tracking = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        29
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._tracking);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._tracking = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        Ok(())
    }
}
