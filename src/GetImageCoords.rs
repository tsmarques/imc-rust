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

/// Message containing the x and y coordinates of object to track in image slave.
#[derive(Default, Clone)]
pub struct GetImageCoords {
    /// Message Header
    pub _header: Header,
    /// Camera identifier.
    pub _camId: u8,
    /// X coordinate of the target in the image frame.
    pub _x: u16,
    /// Y coordinate of the target in the image frame.
    pub _y: u16,
}

impl Message for GetImageCoords {
    fn new() -> GetImageCoords {
        

        GetImageCoords {
            _header: Header::new(896),
            _camId: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        896
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        896
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
        self._header = Header::new(896);
        self._camId = Default::default();
        self._x = Default::default();
        self._y = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._camId);
        bfr.put_u16_le(self._x);
        bfr.put_u16_le(self._y);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._camId = bfr.get_u8();
        self._x = bfr.get_u16_le();
        self._y = bfr.get_u16_le();
        Ok(())
    }
}
