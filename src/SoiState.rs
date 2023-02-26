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
    /// Executing
    SOISTATE_EXEC = 1,
    /// Idle
    SOISTATE_IDLE = 2,
    /// Inactive
    SOISTATE_INACTIVE = 3,
}

#[derive(Default, Clone)]
pub struct SoiState {
    /// Message Header
    pub _header: Header,
    pub _state: u8,
    pub _plan_id: u16,
    pub _wpt_id: u8,
    pub _settings_chk: u16,
}

impl Message for SoiState {
    fn new() -> SoiState {
        

        SoiState {
            _header: Header::new(853),
            _state: Default::default(),
            _plan_id: Default::default(),
            _wpt_id: Default::default(),
            _settings_chk: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        853
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        853
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
        self._header = Header::new(853);
        self._state = Default::default();
        self._plan_id = Default::default();
        self._wpt_id = Default::default();
        self._settings_chk = Default::default()
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
        bfr.put_u8(self._state);
        bfr.put_u16_le(self._plan_id);
        bfr.put_u8(self._wpt_id);
        bfr.put_u16_le(self._settings_chk);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        self._plan_id = bfr.get_u16_le();
        self._wpt_id = bfr.get_u8();
        self._settings_chk = bfr.get_u16_le();
        Ok(())
    }
}
