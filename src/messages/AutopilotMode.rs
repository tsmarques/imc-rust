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

/// Autonomy Level
#[allow(non_camel_case_types)]
pub enum AutonomyLevelEnum {
    /// Manual
    AL_MANUAL = 0,
    /// Assisted
    AL_ASSISTED = 1,
    /// Auto
    AL_AUTO = 2,
}

/// Reports autopilot mode.
#[derive(Default, Clone)]
pub struct AutopilotMode {
    /// Message Header
    pub _header: Header,
    /// Current mode autonomy level.
    pub _autonomy: u8,
    /// Current mode name.
    pub _mode: String,
}

impl Message for AutopilotMode {
    fn new() -> AutopilotMode {
        AutopilotMode {
            _header: Header::new(511),
            _autonomy: Default::default(),
            _mode: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        511
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        511
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
        self._header = Header::new(511);
        self._autonomy = Default::default();
        self._mode = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._mode.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._autonomy);
        serialize_bytes!(bfr, self._mode.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._autonomy = bfr.get_u8();
        deserialize_string!(bfr, self._mode);
        Ok(())
    }
}
