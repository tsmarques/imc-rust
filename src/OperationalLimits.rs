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

/// Definition of operational limits.
#[derive(Default)]
pub struct OperationalLimits {
    /// Message Header.
    pub _header: Header,
    /// Field Indicator Mask.
    pub _mask: u8,
    /// Maximum Depth.
    pub _max_depth: f32,
    /// Minimum Altitude.
    pub _min_altitude: f32,
    /// Maximum Altitude.
    pub _max_altitude: f32,
    /// Minimum Speed.
    pub _min_speed: f32,
    /// Maximum Speed.
    pub _max_speed: f32,
    /// Maximum Vertical Rate.
    pub _max_vrate: f32,
    /// Area -- WGS-84 Latitude.
    pub _lat: f64,
    /// Area -- WGS-84 Longitude.
    pub _lon: f64,
    /// Area -- Orientation.
    pub _orientation: f32,
    /// Area -- Width.
    pub _width: f32,
    /// Area -- Length.
    pub _length: f32,
}

impl Message for OperationalLimits {
    fn new() -> OperationalLimits {
        let msg = OperationalLimits {
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
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        504
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        504
    }

    fn get_header(&mut self) -> &mut Header {
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
