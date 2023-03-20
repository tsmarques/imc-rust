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

/// Request creating a new session with this remote peer. Example
/// session sequence is shown in the following diagram.
/// .. figure:: ../images/session_sequence.png
/// :align:  center
#[derive(Default, Clone)]
pub struct CreateSession {
    /// Message Header
    pub _header: Header,
    /// Session timeout, in seconds. If no messages are received from
    /// the remote peer, the session will be closed after this ammount
    /// of seconds have ellapsed.
    pub _timeout: u32,
}

impl Message for CreateSession {
    fn new() -> CreateSession {
        CreateSession {
            _header: Header::new(806),
            _timeout: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        806
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        806
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
        self._header = Header::new(806);
        self._timeout = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._timeout);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u32_le();
        Ok(())
    }
}
