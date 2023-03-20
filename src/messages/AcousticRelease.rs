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

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Open
    AROP_OPEN = 0,
    /// Close
    AROP_CLOSE = 1,
}

/// Request a system (local or remote) to activate its acoustic release.
#[derive(Default, Clone)]
pub struct AcousticRelease {
    /// Message Header
    pub _header: Header,
    /// The name of the system that should execute an acoustic release.
    pub _system: String,
    pub _op: u8,
}

impl Message for AcousticRelease {
    fn new() -> AcousticRelease {
        AcousticRelease {
            _header: Header::new(217),
            _system: Default::default(),
            _op: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        217
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        217
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
        self._header = Header::new(217);
        self._system = Default::default();
        self._op = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._system.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._system.as_bytes());
        bfr.put_u8(self._op);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._system);
        self._op = bfr.get_u8();
        Ok(())
    }
}
