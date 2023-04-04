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

use crate::Header;
use crate::Message;

/// Beam configuration of the device.
#[derive(Default, Clone)]
pub struct BeamConfig {
    /// Message Header
    pub _header: Header,
    /// Beam width of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_width: f32,
    /// Beam height of the instrument. A negative number denotes that
    /// this information is not available or is not applicable.
    pub _beam_height: f32,
}

impl Message for BeamConfig {
    fn new() -> BeamConfig {
        BeamConfig {
            _header: Header::new(283),
            _beam_width: Default::default(),
            _beam_height: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        283
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        283
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
        self._header = Header::new(283);
        self._beam_width = Default::default();
        self._beam_height = Default::default()
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
        bfr.put_f32_le(self._beam_width);
        bfr.put_f32_le(self._beam_height);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._beam_width = bfr.get_f32_le();
        self._beam_height = bfr.get_f32_le();
        Ok(())
    }
}