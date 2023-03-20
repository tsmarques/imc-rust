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
    /// Power Down
    POP_PWR_DOWN = 0,
    /// Power Down in Progress
    POP_PWR_DOWN_IP = 1,
    /// Power Down Aborted
    POP_PWR_DOWN_ABORTED = 2,
    /// Schedule Power Down
    POP_SCHED_PWR_DOWN = 3,
    /// Power Up
    POP_PWR_UP = 4,
    /// Power Up in Progress
    POP_PWR_UP_IP = 5,
    /// Schedule Power Up
    POP_SCHED_PWR_UP = 6,
}

/// This message allows controlling the system's power lines.
#[derive(Default, Clone)]
pub struct PowerOperation {
    /// Message Header
    pub _header: Header,
    /// Operation type.
    pub _op: u8,
    /// Time remaining to complete operation.
    pub _time_remain: f32,
    /// Scheduled time of operation.
    pub _sched_time: f64,
}

impl Message for PowerOperation {
    fn new() -> PowerOperation {
        PowerOperation {
            _header: Header::new(308),
            _op: Default::default(),
            _time_remain: Default::default(),
            _sched_time: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        308
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        308
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
        self._header = Header::new(308);
        self._op = Default::default();
        self._time_remain = Default::default();
        self._sched_time = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        13
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_f32_le(self._time_remain);
        bfr.put_f64_le(self._sched_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._time_remain = bfr.get_f32_le();
        self._sched_time = bfr.get_f64_le();
        Ok(())
    }
}
