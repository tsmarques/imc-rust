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

/// Request an acoustic modem driver to measure the distance to another system.
#[derive(Default, Clone)]
pub struct UamTxRange {
    /// Message Header
    pub _header: Header,
    /// A sequence identifier that should be incremented for each
    /// request. This number will then be used to issue transmission
    /// status updates via the message UamTxStatus.
    pub _seq: u16,
    /// The canonical name of the target system.
    pub _sys_dst: String,
    /// Maximum amount of time to wait for a reply.
    pub _timeout: f32,
}

impl Message for UamTxRange {
    fn new() -> UamTxRange {
        UamTxRange {
            _header: Header::new(818),
            _seq: Default::default(),
            _sys_dst: Default::default(),
            _timeout: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        818
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        818
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
        self._header = Header::new(818);
        self._seq = Default::default();
        self._sys_dst = Default::default();
        self._timeout = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sys_dst.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_f32_le(self._timeout);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u16_le();
        deserialize_string!(bfr, self._sys_dst);
        self._timeout = bfr.get_f32_le();
        Ok(())
    }
}
