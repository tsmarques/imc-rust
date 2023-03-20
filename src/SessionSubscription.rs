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

#[derive(Default, Clone)]
pub struct SessionSubscription {
    /// Message Header
    pub _header: Header,
    pub _sessid: u32,
    /// Comma-separated list of messages to subscribe. Example:
    /// "EstimatedState,EulerAngles,Temperature"
    pub _messages: String,
}

impl Message for SessionSubscription {
    fn new() -> SessionSubscription {
        SessionSubscription {
            _header: Header::new(808),
            _sessid: Default::default(),
            _messages: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        808
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        808
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
        self._header = Header::new(808);
        self._sessid = Default::default();
        self._messages = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._messages.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._sessid);
        serialize_bytes!(bfr, self._messages.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._sessid = bfr.get_u32_le();
        deserialize_string!(bfr, self._messages);
        Ok(())
    }
}
