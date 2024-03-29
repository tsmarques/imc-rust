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
    /// Execute Sync.
    COP_SYNC_EXEC = 0,
    /// Request Sync.
    COP_SYNC_REQUEST = 1,
    /// Sync. Started
    COP_SYNC_STARTED = 2,
    /// Sync. done
    COP_SYNC_DONE = 3,
    /// Set Timezone
    COP_SET_TZ = 4,
    /// Timezone Setup
    COP_SET_TZ_DONE = 5,
}

/// Clock control.
#[derive(Default, Clone)]
pub struct ClockControl {
    /// Message Header
    pub _header: Header,
    /// Operation to perform.
    pub _op: u8,
    /// Clock value (Epoch time).
    pub _clock: f64,
    /// Timezone.
    pub _tz: i8,
}

impl Message for ClockControl {
    fn new() -> ClockControl {
        ClockControl {
            _header: Header::new(106),
            _op: Default::default(),
            _clock: Default::default(),
            _tz: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        106
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        106
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
        self._header = Header::new(106);
        self._op = Default::default();
        self._clock = Default::default();
        self._tz = Default::default()
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
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._clock);
        bfr.put_i8(self._tz);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._clock = bfr.get_f64_le();
        self._tz = bfr.get_i8();
        Ok(())
    }
}
