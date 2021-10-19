//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::Header::Header;
use crate::Message::*;

/// Query the activation/deactivation state of an entity. The
/// recipient shall reply with an EntityActivationState message.
#[derive(Default)]
pub struct QueryEntityActivationState {
    /// Message Header
    pub _header: Header,
}

impl Message for QueryEntityActivationState {
    fn new() -> QueryEntityActivationState {
        let msg = QueryEntityActivationState {
            _header: Header::new(15),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        15
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        15
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(15)
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, _bfr: &mut bytes::BytesMut) {}

    fn deserialize_fields(&mut self, _bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        Ok(())
    }
}
