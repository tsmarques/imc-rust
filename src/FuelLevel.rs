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
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Report of fuel level.
#[derive(Default)]
pub struct FuelLevel {
    /// Message Header
    pub _header: Header,
    /// Fuel level percentage of the system.
    pub _value: f32,
    /// Percentage level of confidence in the estimation of the amount
    /// of energy in the batteries.
    pub _confidence: f32,
    /// Operation mode name and the estimated time available in that
    /// mode in hours. Example: "Motion=1.5"
    pub _opmodes: String,
}

impl Message for FuelLevel {
    fn new() -> FuelLevel {
        let msg = FuelLevel {
            _header: Header::new(279),
            _value: Default::default(),
            _confidence: Default::default(),
            _opmodes: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        279
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        279
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(279);
        self._value = Default::default();
        self._confidence = Default::default();
        self._opmodes = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._opmodes.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_f32_le(self._confidence);
        serialize_bytes!(bfr, self._opmodes.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();
        self._confidence = bfr.get_f32_le();
        deserialize_string!(bfr, self._opmodes);
        Ok(())
    }
}
