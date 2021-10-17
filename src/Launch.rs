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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// A "Launch" is a maneuver specifying a movement of the vehicle to a
/// target waypoint after being launched from a ship or pier.
/// The waypoint is described by the WGS-84 waypoint coordinate and
/// target Z reference.
/// Mandatory parameters defined for a "Launch" are
/// timeout, speed and speed units.
#[derive(Default)]
pub struct Launch {
    /// Message Header.
    pub _header: Header,
    /// Timeout.
    pub _timeout: u16,
    /// Latitude WGS-84.
    pub _lat: f64,
    /// Longitude WGS-84.
    pub _lon: f64,
    /// Z Reference.
    pub _z: f32,
    /// Z Units.
    pub _z_units: u8,
    /// Speed.
    pub _speed: f32,
    /// Speed Units.
    pub _speed_units: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Launch {
    fn new() -> Launch
    where
        Self: Sized,
    {
        let msg = Launch {
            _header: Header::new(485),
            _timeout: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        485
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        485
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(485);
        self._timeout = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        28
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
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
