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

/// Report of GPS navigation data.
#[derive(Default, Clone)]
pub struct GpsNavData {
    /// Message Header
    pub _header: Header,
    /// GPS Millisecond Time of Week.
    pub _itow: u32,
    /// Latitude.
    pub _lat: f64,
    /// Longitude.
    pub _lon: f64,
    /// Height Above Ellipsoid.
    pub _height_ell: f32,
    /// Height Above Sea Level.
    pub _height_sea: f32,
    /// Horizontal Accuracy Estimate.
    pub _hacc: f32,
    /// Vertical Accuracy Estimate.
    pub _vacc: f32,
    /// NED North Velocity.
    pub _vel_n: f32,
    /// NED East Velocity.
    pub _vel_e: f32,
    /// NED Down Velocity.
    pub _vel_d: f32,
    /// NED Down Velocity.
    pub _speed: f32,
    /// NED Down Velocity.
    pub _gspeed: f32,
    /// NED Down Velocity.
    pub _heading: f32,
    /// NED Down Velocity.
    pub _sacc: f32,
    /// Course / Heading Accuracy Estimate.
    pub _cacc: f32,
}

impl Message for GpsNavData {
    fn new() -> GpsNavData {
        GpsNavData {
            _header: Header::new(280),
            _itow: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height_ell: Default::default(),
            _height_sea: Default::default(),
            _hacc: Default::default(),
            _vacc: Default::default(),
            _vel_n: Default::default(),
            _vel_e: Default::default(),
            _vel_d: Default::default(),
            _speed: Default::default(),
            _gspeed: Default::default(),
            _heading: Default::default(),
            _sacc: Default::default(),
            _cacc: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        280
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        280
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
        self._header = Header::new(280);
        self._itow = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._height_ell = Default::default();
        self._height_sea = Default::default();
        self._hacc = Default::default();
        self._vacc = Default::default();
        self._vel_n = Default::default();
        self._vel_e = Default::default();
        self._vel_d = Default::default();
        self._speed = Default::default();
        self._gspeed = Default::default();
        self._heading = Default::default();
        self._sacc = Default::default();
        self._cacc = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        68
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._itow);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height_ell);
        bfr.put_f32_le(self._height_sea);
        bfr.put_f32_le(self._hacc);
        bfr.put_f32_le(self._vacc);
        bfr.put_f32_le(self._vel_n);
        bfr.put_f32_le(self._vel_e);
        bfr.put_f32_le(self._vel_d);
        bfr.put_f32_le(self._speed);
        bfr.put_f32_le(self._gspeed);
        bfr.put_f32_le(self._heading);
        bfr.put_f32_le(self._sacc);
        bfr.put_f32_le(self._cacc);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._itow = bfr.get_u32_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height_ell = bfr.get_f32_le();
        self._height_sea = bfr.get_f32_le();
        self._hacc = bfr.get_f32_le();
        self._vacc = bfr.get_f32_le();
        self._vel_n = bfr.get_f32_le();
        self._vel_e = bfr.get_f32_le();
        self._vel_d = bfr.get_f32_le();
        self._speed = bfr.get_f32_le();
        self._gspeed = bfr.get_f32_le();
        self._heading = bfr.get_f32_le();
        self._sacc = bfr.get_f32_le();
        self._cacc = bfr.get_f32_le();
        Ok(())
    }
}
