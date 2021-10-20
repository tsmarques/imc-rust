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

/// Report of external pressure.
#[derive(Default)]
pub struct Pressure {
    /// Message Header
    pub _header: Header,
    /// The value of the pressure as measured by the sensor.
    pub _value: f64,
}

impl Message for Pressure {
    fn new() -> Pressure {
        let msg = Pressure {
            _header: Header::new(264),
            _value: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        264
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        264
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(264);
        self._value = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f64_le();
        Ok(())
    }
}
