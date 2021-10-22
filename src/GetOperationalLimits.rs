//###########################################################################
// Copyright 2021 OceanScan - Marine Systems & Technology, Lda.             #
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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::Header::Header;
use crate::Message::*;

/// Command to obtain the operational limits in use by the vehicle.
#[derive(Default)]
pub struct GetOperationalLimits {
    /// Message Header
    pub _header: Header,
}

impl Message for GetOperationalLimits {
    fn new() -> GetOperationalLimits {
        let msg = GetOperationalLimits {
            _header: Header::new(505),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        505
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        505
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(505)
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
