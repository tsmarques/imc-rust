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

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Promiscuous
    pub const URF_PROMISCUOUS: u32 = 0x01;
    /// Delayed
    pub const URF_DELAYED: u32 = 0x02;
}

/// This message shall be dispatched by acoustic modem drivers each time
/// a data frame is received over the acoustic channel.
#[derive(Default, Clone)]
pub struct UamRxFrame {
    /// Message Header
    pub _header: Header,
    /// The canonical name of the node that transmitted the data frame. If
    /// this name cannot be resolved the string 'unknown' shall be used.
    pub _sys_src: String,
    /// The canonical name of the destination node of the data frame. If
    /// this name cannot be resolved the string 'unknown' shall be used.
    pub _sys_dst: String,
    /// Reception flags.
    pub _flags: u8,
    /// The actual received data frame.
    pub _data: Vec<u8>,
}

impl Message for UamRxFrame {
    fn new() -> UamRxFrame {
        UamRxFrame {
            _header: Header::new(815),
            _sys_src: Default::default(),
            _sys_dst: Default::default(),
            _flags: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        815
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        815
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
        self._header = Header::new(815);
        self._sys_src = Default::default();
        self._sys_dst = Default::default();
        self._flags = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sys_src.len() + 2;
        dyn_size += self._sys_dst.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._sys_src.as_bytes());
        serialize_bytes!(bfr, self._sys_dst.as_bytes());
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._sys_src);
        deserialize_string!(bfr, self._sys_dst);
        self._flags = bfr.get_u8();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
