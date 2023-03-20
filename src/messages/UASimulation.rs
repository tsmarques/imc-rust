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

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Data Transmission
    UAS_DATA = 0,
    /// Ping
    UAS_PING = 1,
    /// Ping Reply
    UAS_PING_REPLY = 2,
}

/// Underwater acoustics simulation request.
#[derive(Default, Clone)]
pub struct UASimulation {
    /// Message Header
    pub _header: Header,
    /// Type of request.
    pub _type: u8,
    /// Transmission speed.
    pub _speed: u16,
    /// Data for transmission requests.
    pub _data: Vec<u8>,
}

impl Message for UASimulation {
    fn new() -> UASimulation {
        UASimulation {
            _header: Header::new(52),
            _type: Default::default(),
            _speed: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        52
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        52
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
        self._header = Header::new(52);
        self._type = Default::default();
        self._speed = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._speed);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._speed = bfr.get_u16_le();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
