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

/// Validity.
#[allow(non_camel_case_types)]
pub mod ValidityBits {
    /// X component is valid.
    pub const VAL_VEL_X: u32 = 0x01;
    /// Y component is valid.
    pub const VAL_VEL_Y: u32 = 0x02;
    /// Z component is valid.
    pub const VAL_VEL_Z: u32 = 0x04;
}

/// Vector quantifying the direction and magnitude of the measured
/// velocity relative to the water that a device is exposed to.
#[derive(Default)]
pub struct WaterVelocity {
    /// Message Header.
    pub _header: Header,
    /// Validity.
    pub _validity: u8,
    /// X.
    pub _x: f64,
    /// Y.
    pub _y: f64,
    /// Z.
    pub _z: f64,
}

impl Message for WaterVelocity {
    fn new() -> WaterVelocity {
        let msg = WaterVelocity {
            _header: Header::new(260),
            _validity: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        260
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        260
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(260);
        self._validity = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        25
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._validity);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u8();
        self._x = bfr.get_f64_le();
        self._y = bfr.get_f64_le();
        self._z = bfr.get_f64_le();
        Ok(())
    }
}
