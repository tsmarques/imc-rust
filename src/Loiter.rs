//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Loiter Type
#[allow(non_camel_case_types)]
pub enum LoiterTypeEnum {
    /// Default
    LT_DEFAULT = 0,
    /// Circular
    LT_CIRCULAR = 1,
    /// Race track
    LT_RACETRACK = 2,
    /// Figure 8
    LT_EIGHT = 3,
    /// Hover
    LT_HOVER = 4,
}

/// Direction
#[allow(non_camel_case_types)]
pub enum DirectionEnum {
    /// Vehicle Dependent
    LD_VDEP = 0,
    /// Clockwise
    LD_CLOCKW = 1,
    /// Counter Clockwise
    LD_CCLOCKW = 2,
    /// Into the wind/current
    LD_IWINDCURR = 3,
}

/// The Loiter maneuver makes the vehicle circle around a specific
/// waypoint with fixed depth reference.
#[derive(Default, Clone)]
pub struct Loiter {
    /// Message Header
    pub _header: Header,
    /// The timeout indicates the time that an error should occur if
    /// exceeded.
    pub _timeout: u16,
    /// WGS-84 Latitude coordinate.
    pub _lat: f64,
    /// WGS-84 Longitude coordinate.
    pub _lon: f64,
    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
    /// The duration of this maneuver. Use '0' for unlimited duration
    /// time.
    pub _duration: u16,
    /// Maneuver speed reference.
    pub _speed: f32,
    /// Speed units.
    pub _speed_units: u8,
    /// Loiter maneuver type.
    pub _type: u8,
    /// Radius of the maneuver.
    pub _radius: f32,
    /// Length of the maneuver.
    pub _length: f32,
    /// Bearing of the maneuver.
    pub _bearing: f64,
    /// Desired direction.
    pub _direction: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Loiter {
    fn new() -> Loiter {
        let msg = Loiter {
            _header: Header::new(453),
            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _duration: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _type: Default::default(),
            _radius: Default::default(),
            _length: Default::default(),
            _bearing: Default::default(),
            _direction: Default::default(),
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        453
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        453
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(453);
        self._timeout = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._duration = Default::default();
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._type = Default::default();
        self._radius = Default::default();
        self._length = Default::default();
        self._bearing = Default::default();
        self._direction = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        48
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
        bfr.put_u16_le(self._duration);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_u8(self._type);
        bfr.put_f32_le(self._radius);
        bfr.put_f32_le(self._length);
        bfr.put_f64_le(self._bearing);
        bfr.put_u8(self._direction);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._duration = bfr.get_u16_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._type = bfr.get_u8();
        self._radius = bfr.get_f32_le();
        self._length = bfr.get_f32_le();
        self._bearing = bfr.get_f64_le();
        self._direction = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
