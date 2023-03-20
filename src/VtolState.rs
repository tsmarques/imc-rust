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

/// State
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Undefined
    VTOL_STATE_UNDEFINED = 0,
    /// Transition to Fixed-Wing
    VTOL_STATE_TRANSITION_TO_FW = 1,
    /// Transition to MultiCopter
    VTOL_STATE_TRANSITION_TO_MC = 2,
    /// MutiCopter
    VTOL_STATE_MC = 3,
    /// Fixed-Wing
    VTOL_STATE_FW = 4,
}

/// Reports VTOL current state.
#[derive(Default, Clone)]
pub struct VtolState {
    /// Message Header
    pub _header: Header,
    pub _state: u8,
}

impl Message for VtolState {
    fn new() -> VtolState {
        VtolState {
            _header: Header::new(519),
            _state: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        519
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        519
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
        self._header = Header::new(519);
        self._state = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        Ok(())
    }
}
