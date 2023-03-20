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

/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::Header;
use crate::Message;

/// Hardware pulse detection.
#[derive(Default, Clone)]
pub struct Pulse {
    /// Message Header
    pub _header: Header,
}

impl Message for Pulse {
    fn new() -> Pulse {
        Pulse {
            _header: Header::new(277),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        277
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        277
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
        self._header = Header::new(277)
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, _bfr: &mut bytes::BytesMut) {}

    fn deserialize_fields(&mut self, _bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        Ok(())
    }
}
