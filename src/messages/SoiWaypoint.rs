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

#[derive(Default, Clone)]
pub struct SoiWaypoint {
    /// Message Header
    pub _header: Header,
    pub _lat: f32,
    pub _lon: f32,
    pub _eta: u32,
    pub _duration: u16,
}

impl Message for SoiWaypoint {
    fn new() -> SoiWaypoint {
        SoiWaypoint {
            _header: Header::new(850),
            _lat: Default::default(),
            _lon: Default::default(),
            _eta: Default::default(),
            _duration: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        850
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        850
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
        self._header = Header::new(850);
        self._lat = Default::default();
        self._lon = Default::default();
        self._eta = Default::default();
        self._duration = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        14
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._lat);
        bfr.put_f32_le(self._lon);
        bfr.put_u32_le(self._eta);
        bfr.put_u16_le(self._duration);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f32_le();
        self._lon = bfr.get_f32_le();
        self._eta = bfr.get_u32_le();
        self._duration = bfr.get_u16_le();
        Ok(())
    }
}
