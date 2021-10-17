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

/// This message contains the WGS-84 position of a target computed using
/// USBL.
#[derive(Default)]
pub struct UsblFix {
    /// Message Header.
    pub _header: Header,
    /// Target.
    pub _target: u16,
    /// Latitude (WGS-84).
    pub _lat: f64,
    /// Longitude (WGS-84).
    pub _lon: f64,
    /// Z Units.
    pub _z_units: u8,
    /// Z Reference.
    pub _z: f32,
}

impl Message for UsblFix {
    fn new() -> UsblFix {
        let msg = UsblFix {
            _header: Header::new(892),
            _target: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z_units: 0_u8,
            _z: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        892
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        892
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(892);
        self._target = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z_units = 0_u8;
        self._z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        23
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._target);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._target = bfr.get_u16_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z_units = bfr.get_u8();
        self._z = bfr.get_f32_le();
        Ok(())
    }
}
