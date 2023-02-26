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
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Gain
#[allow(non_camel_case_types)]
pub enum GainEnum {
    /// x1
    GAIN_X1 = 0,
    /// x10
    GAIN_X10 = 1,
    /// x100
    GAIN_X100 = 2,
}

/// Readings from SADC board.
#[derive(Default, Clone)]
pub struct SadcReadings {
    /// Message Header
    pub _header: Header,
    /// Channel of SADC to read.
    pub _channel: i8,
    /// Value raw of sadc channel.
    pub _value: i32,
    /// Gain value of readings.
    pub _gain: u8,
}

impl Message for SadcReadings {
    fn new() -> SadcReadings {
        

        SadcReadings {
            _header: Header::new(907),
            _channel: Default::default(),
            _value: Default::default(),
            _gain: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        907
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        907
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(907);
        self._channel = Default::default();
        self._value = Default::default();
        self._gain = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_i8(self._channel);
        bfr.put_i32_le(self._value);
        bfr.put_u8(self._gain);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._channel = bfr.get_i8();
        self._value = bfr.get_i32_le();
        self._gain = bfr.get_u8();
        Ok(())
    }
}
