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

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Square Curve
    pub const FLG_SQUARE_CURVE: u32 = 0x01;
    /// First Curve Right
    pub const FLG_CURVE_RIGHT: u32 = 0x02;
}

/// Rows coverage (i.e: lawn mower type maneuver) but with adaptive cover
#[derive(Default, Clone)]
pub struct RowsCoverage {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude of target waypoint.
    pub _lat: f64,
    /// WGS-84 Longitude of target waypoint.
    pub _lon: f64,
    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
    /// Maneuver speed reference.
    pub _speed: f32,
    /// Speed units.
    pub _speed_units: u8,
    /// Rows bearing angle.
    pub _bearing: f64,
    /// Rows cross angle reference.
    pub _cross_angle: f64,
    /// Width of the maneuver.
    pub _width: f32,
    /// Length of the maneuver.
    pub _length: f32,
    /// Desired curve offset.
    pub _coff: u8,
    /// Angular aperture of the sensor (looking downwards).
    pub _angAperture: f32,
    /// Maximum range of the sensor (in meters downwards from the vehicle's position).
    pub _range: u16,
    /// Amount of overlap among different transect surveys.
    pub _overlap: u8,
    /// Maneuver optional flags.
    pub _flags: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for RowsCoverage {
    fn new() -> RowsCoverage {
        

        RowsCoverage {
            _header: Header::new(488),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _bearing: Default::default(),
            _cross_angle: Default::default(),
            _width: Default::default(),
            _length: Default::default(),
            _coff: Default::default(),
            _angAperture: 2.094395_f32,
            _range: Default::default(),
            _overlap: 10_u8,
            _flags: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        488
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        488
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
        self._header = Header::new(488);
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._bearing = Default::default();
        self._cross_angle = Default::default();
        self._width = Default::default();
        self._length = Default::default();
        self._coff = Default::default();
        self._angAperture = 2.094395_f32;
        self._range = Default::default();
        self._overlap = 10_u8;
        self._flags = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        59
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
        bfr.put_f64_le(self._bearing);
        bfr.put_f64_le(self._cross_angle);
        bfr.put_f32_le(self._width);
        bfr.put_f32_le(self._length);
        bfr.put_u8(self._coff);
        bfr.put_f32_le(self._angAperture);
        bfr.put_u16_le(self._range);
        bfr.put_u8(self._overlap);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._bearing = bfr.get_f64_le();
        self._cross_angle = bfr.get_f64_le();
        self._width = bfr.get_f32_le();
        self._length = bfr.get_f32_le();
        self._coff = bfr.get_u8();
        self._angAperture = bfr.get_f32_le();
        self._range = bfr.get_u16_le();
        self._overlap = bfr.get_u8();
        self._flags = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
