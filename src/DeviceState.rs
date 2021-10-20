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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Location of a specific device in the system infrastructure.
#[derive(Default)]
pub struct DeviceState {
    /// Message Header
    pub _header: Header,
    /// Device's position over the X axis.
    pub _x: f32,
    /// Device's position over the Y axis.
    pub _y: f32,
    /// Device's position over the Z axis.
    pub _z: f32,
    /// Device's rotation over the X axis.
    pub _phi: f32,
    /// Device's rotation over the Y axis.
    pub _theta: f32,
    /// Device's rotation over the Z axis.
    pub _psi: f32,
}

impl Message for DeviceState {
    fn new() -> DeviceState {
        let msg = DeviceState {
            _header: Header::new(282),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        282
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        282
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(282);
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        Ok(())
    }
}
