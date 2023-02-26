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

/// Properties of a PWM signal channel.
#[derive(Default, Clone)]
pub struct PWM {
    /// Message Header
    pub _header: Header,
    /// PWM channel identifier.
    pub _id: u8,
    /// The total period of the PWM signal (sum of active and inactive
    /// time of the PWM).
    pub _period: u32,
    /// The active time of the PWM signal. The duty cycle value is
    /// less or equal to the period.
    pub _duty_cycle: u32,
}

impl Message for PWM {
    fn new() -> PWM {
        

        PWM {
            _header: Header::new(316),
            _id: Default::default(),
            _period: Default::default(),
            _duty_cycle: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        316
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        316
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
        self._header = Header::new(316);
        self._id = Default::default();
        self._period = Default::default();
        self._duty_cycle = Default::default()
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
        bfr.put_u8(self._id);
        bfr.put_u32_le(self._period);
        bfr.put_u32_le(self._duty_cycle);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._id = bfr.get_u8();
        self._period = bfr.get_u32_le();
        self._duty_cycle = bfr.get_u32_le();
        Ok(())
    }
}
