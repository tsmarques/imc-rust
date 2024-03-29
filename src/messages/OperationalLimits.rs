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

/// Definition of operational limits.
#[derive(Default, Clone)]
pub struct OperationalLimits {
    /// Message Header
    pub _header: Header,
    pub _mask: u8,
    pub _max_depth: f32,
    pub _min_altitude: f32,
    pub _max_altitude: f32,
    pub _min_speed: f32,
    pub _max_speed: f32,
    pub _max_vrate: f32,
    pub _lat: f64,
    pub _lon: f64,
    pub _orientation: f32,
    pub _width: f32,
    pub _length: f32,
}

impl Message for OperationalLimits {
    fn new() -> OperationalLimits {
        OperationalLimits {
            _header: Header::new(504),
            _mask: Default::default(),
            _max_depth: Default::default(),
            _min_altitude: Default::default(),
            _max_altitude: Default::default(),
            _min_speed: Default::default(),
            _max_speed: Default::default(),
            _max_vrate: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _orientation: Default::default(),
            _width: Default::default(),
            _length: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        504
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        504
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
        self._header = Header::new(504);
        self._mask = Default::default();
        self._max_depth = Default::default();
        self._min_altitude = Default::default();
        self._max_altitude = Default::default();
        self._min_speed = Default::default();
        self._max_speed = Default::default();
        self._max_vrate = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._orientation = Default::default();
        self._width = Default::default();
        self._length = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        53
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._mask);
        bfr.put_f32_le(self._max_depth);
        bfr.put_f32_le(self._min_altitude);
        bfr.put_f32_le(self._max_altitude);
        bfr.put_f32_le(self._min_speed);
        bfr.put_f32_le(self._max_speed);
        bfr.put_f32_le(self._max_vrate);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._orientation);
        bfr.put_f32_le(self._width);
        bfr.put_f32_le(self._length);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._mask = bfr.get_u8();
        self._max_depth = bfr.get_f32_le();
        self._min_altitude = bfr.get_f32_le();
        self._max_altitude = bfr.get_f32_le();
        self._min_speed = bfr.get_f32_le();
        self._max_speed = bfr.get_f32_le();
        self._max_vrate = bfr.get_f32_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._orientation = bfr.get_f32_le();
        self._width = bfr.get_f32_le();
        self._length = bfr.get_f32_le();
        Ok(())
    }
}
