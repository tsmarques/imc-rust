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

/// State
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Not Configured
    ECS_NOT_CONFIGURED = 0,
    /// Disabled
    ECS_DISABLED = 1,
    /// Enabled
    ECS_ENABLED = 2,
    /// Armed
    ECS_ARMED = 3,
    /// Active
    ECS_ACTIVE = 4,
    /// Stopping
    ECS_STOPPING = 5,
}

#[derive(Default, Clone)]
pub struct EmergencyControlState {
    /// Message Header
    pub _header: Header,
    pub _state: u8,
    pub _plan_id: String,
    pub _comm_level: u8,
}

impl Message for EmergencyControlState {
    fn new() -> EmergencyControlState {
        EmergencyControlState {
            _header: Header::new(555),
            _state: Default::default(),
            _plan_id: Default::default(),
            _comm_level: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        555
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        555
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
        self._header = Header::new(555);
        self._state = Default::default();
        self._plan_id = Default::default();
        self._comm_level = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u8(self._comm_level);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        deserialize_string!(bfr, self._plan_id);
        self._comm_level = bfr.get_u8();
        Ok(())
    }
}
