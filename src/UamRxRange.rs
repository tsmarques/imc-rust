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

/// Acoustic range measurement.
#[derive(Default, Clone)]
pub struct UamRxRange {
    /// Message Header
    pub _header: Header,
    /// The sequence identifier of the ranging request.
    pub _seq: u16,
    /// The canonical name of the ranged system.
    pub _sys: String,
    /// The actual range. Negative values denote invalid measurements.
    pub _value: f32,
}

impl Message for UamRxRange {
    fn new() -> UamRxRange {
        UamRxRange {
            _header: Header::new(817),
            _seq: Default::default(),
            _sys: Default::default(),
            _value: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        817
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        817
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(817);
        self._seq = Default::default();
        self._sys = Default::default();
        self._value = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sys.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys.as_bytes());
        bfr.put_f32_le(self._value);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u16_le();
        deserialize_string!(bfr, self._sys);
        self._value = bfr.get_f32_le();
        Ok(())
    }
}
