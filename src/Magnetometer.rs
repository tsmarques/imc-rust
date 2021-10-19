//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Direction
#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    /// Clockwise First
    MD_CLOCKW_FIRST = 0,
    /// Counter Clockwise First
    MD_CCLOCKW_FIRST = 1,
}

/// Magnetometer calibration maneuver (i.e: one square pattern
/// in one direction, a second square in the opposite direction)
#[derive(Default)]
pub struct Magnetometer {
    /// Message Header
    pub _header: Header,
    /// The amount of time the maneuver is allowed to run.
    pub _timeout: u16,
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
    /// Width of the maneuver.
    pub _width: f32,
    /// Desired direction.
    pub _direction: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Magnetometer {
    fn new() -> Magnetometer {
        let msg = Magnetometer {
            _header: Header::new(499),
            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _bearing: Default::default(),
            _width: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        499
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        499
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(499);
        self._timeout = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._bearing = Default::default();
        self._width = Default::default();
        self._direction = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        41
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f64_le(self._bearing);
        bfr.put_f32_le(self._width);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._bearing = bfr.get_f64_le();
        self._width = bfr.get_f32_le();
        self._direction = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
