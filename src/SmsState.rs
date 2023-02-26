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
    /// Accepted
    SMS_ACCEPTED = 0,
    /// Rejected
    SMS_REJECTED = 1,
    /// Interrupted
    SMS_INTERRUPTED = 2,
    /// Completed
    SMS_COMPLETED = 3,
    /// Idle
    SMS_IDLE = 4,
    /// Transmitting
    SMS_TRANSMITTING = 5,
    /// Receiving
    SMS_RECEIVING = 6,
}

#[derive(Default, Clone)]
pub struct SmsState {
    /// Message Header
    pub _header: Header,
    /// Sequence number.
    pub _seq: u32,
    /// Current state of an SMS transaction.
    pub _state: u8,
    pub _error: String,
}

impl Message for SmsState {
    fn new() -> SmsState {
        

        SmsState {
            _header: Header::new(159),
            _seq: Default::default(),
            _state: Default::default(),
            _error: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        159
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        159
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
        self._header = Header::new(159);
        self._seq = Default::default();
        self._state = Default::default();
        self._error = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._seq);
        bfr.put_u8(self._state);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u32_le();
        self._state = bfr.get_u8();
        deserialize_string!(bfr, self._error);
        Ok(())
    }
}
