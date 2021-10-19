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

/// Component of incremetal orientation vector over a period of time.
#[derive(Default)]
pub struct EulerAnglesDelta {
    /// Message Header
    pub _header: Header,
    /// The device time.
    pub _time: f64,
    /// X component.
    pub _x: f64,
    /// Y component.
    pub _y: f64,
    /// Z component.
    pub _z: f64,
    /// Period of time of the orientation vector increments.
    pub _timestep: f32,
}

impl Message for EulerAnglesDelta {
    fn new() -> EulerAnglesDelta {
        let msg = EulerAnglesDelta {
            _header: Header::new(255),
            _time: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _timestep: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        255
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        255
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(255);
        self._time = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._timestep = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        36
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
        bfr.put_f32_le(self._timestep);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._time = bfr.get_f64_le();
        self._x = bfr.get_f64_le();
        self._y = bfr.get_f64_le();
        self._z = bfr.get_f64_le();
        self._timestep = bfr.get_f32_le();
        Ok(())
    }
}
