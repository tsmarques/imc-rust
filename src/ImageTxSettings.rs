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
pub struct ImageTxSettings {
    /// Message Header
    pub _header: Header,
    pub _fps: u8,
    pub _quality: u8,
    pub _reps: u8,
    pub _tsize: u8,
}

impl Message for ImageTxSettings {
    fn new() -> ImageTxSettings {
        

        ImageTxSettings {
            _header: Header::new(703),
            _fps: Default::default(),
            _quality: Default::default(),
            _reps: Default::default(),
            _tsize: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        703
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        703
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
        self._header = Header::new(703);
        self._fps = Default::default();
        self._quality = Default::default();
        self._reps = Default::default();
        self._tsize = Default::default()
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
        bfr.put_u8(self._fps);
        bfr.put_u8(self._quality);
        bfr.put_u8(self._reps);
        bfr.put_u8(self._tsize);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._fps = bfr.get_u8();
        self._quality = bfr.get_u8();
        self._reps = bfr.get_u8();
        self._tsize = bfr.get_u8();
        Ok(())
    }
}
