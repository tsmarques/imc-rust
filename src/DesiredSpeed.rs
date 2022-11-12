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

/// Desired Speed reference value for the control layer.
#[derive(Default, Clone)]
pub struct DesiredSpeed {
    /// Message Header
    pub _header: Header,
    /// The value of the desired speed, in the scale specified by the
    /// "Speed Units" field.
    pub _value: f64,
    /// Indicates the units used for the speed value.
    pub _speed_units: u8,
}

impl Message for DesiredSpeed {
    fn new() -> DesiredSpeed {
        let msg = DesiredSpeed {
            _header: Header::new(402),
            _value: Default::default(),
            _speed_units: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        402
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        402
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(402);
        self._value = Default::default();
        self._speed_units = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._value);
        bfr.put_u8(self._speed_units);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f64_le();
        self._speed_units = bfr.get_u8();
        Ok(())
    }
}
