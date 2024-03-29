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

/// This message contains information, collected using USBL, about the
/// bearing and elevation of a target.
#[derive(Default, Clone)]
pub struct UsblAngles {
    /// Message Header
    pub _header: Header,
    /// Target's IMC address.
    pub _target: u16,
    /// Target's bearing.
    pub _bearing: f32,
    /// Target's elevation.
    pub _elevation: f32,
}

impl Message for UsblAngles {
    fn new() -> UsblAngles {
        UsblAngles {
            _header: Header::new(890),
            _target: Default::default(),
            _bearing: Default::default(),
            _elevation: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        890
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        890
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
        self._header = Header::new(890);
        self._target = Default::default();
        self._bearing = Default::default();
        self._elevation = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        10
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._target);
        bfr.put_f32_le(self._bearing);
        bfr.put_f32_le(self._elevation);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._target = bfr.get_u16_le();
        self._bearing = bfr.get_f32_le();
        self._elevation = bfr.get_f32_le();
        Ok(())
    }
}
