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

/// Message generated when tasks bind to messages.
#[derive(Default, Clone)]
pub struct TransportBindings {
    /// Message Header
    pub _header: Header,
    /// The name of the consumer (e.g. task name).
    pub _consumer: String,
    /// The id of the message to be listened to.
    pub _message_id: u16,
}

impl Message for TransportBindings {
    fn new() -> TransportBindings {
        TransportBindings {
            _header: Header::new(8),
            _consumer: Default::default(),
            _message_id: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        8
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        8
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
        self._header = Header::new(8);
        self._consumer = Default::default();
        self._message_id = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._consumer.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._consumer.as_bytes());
        bfr.put_u16_le(self._message_id);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._consumer);
        self._message_id = bfr.get_u16_le();
        Ok(())
    }
}
