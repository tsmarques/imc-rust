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

/// Encoding
#[allow(non_camel_case_types)]
pub enum EncodingEnum {
    /// One Byte Per Pixel
    ENC_ONE_BYTE_PER_PIXEL = 0,
    /// PNG compressed image
    ENC_PNG = 1,
    /// JPEG compressed image
    ENC_JPEG = 2,
}

/// This message is used to store historic (transmitted afterwards) sonar data.
#[derive(Default, Clone)]
pub struct HistoricSonarData {
    /// Message Header
    pub _header: Header,
    pub _altitude: f32,
    pub _width: f32,
    pub _length: f32,
    pub _bearing: f32,
    /// The number of pixels per line as the data in 'sonar_data' may
    /// correspond to more than one sequential sidescan lines.
    pub _pxl: i16,
    pub _encoding: u8,
    /// Sonar data encoded as in 'encoding'.
    pub _sonar_data: Vec<u8>,
}

impl Message for HistoricSonarData {
    fn new() -> HistoricSonarData {
        

        HistoricSonarData {
            _header: Header::new(109),
            _altitude: Default::default(),
            _width: Default::default(),
            _length: Default::default(),
            _bearing: Default::default(),
            _pxl: Default::default(),
            _encoding: Default::default(),
            _sonar_data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        109
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        109
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
        self._header = Header::new(109);
        self._altitude = Default::default();
        self._width = Default::default();
        self._length = Default::default();
        self._bearing = Default::default();
        self._pxl = Default::default();
        self._encoding = Default::default();
        self._sonar_data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        19
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sonar_data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._altitude);
        bfr.put_f32_le(self._width);
        bfr.put_f32_le(self._length);
        bfr.put_f32_le(self._bearing);
        bfr.put_i16_le(self._pxl);
        bfr.put_u8(self._encoding);
        serialize_bytes!(bfr, self._sonar_data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._altitude = bfr.get_f32_le();
        self._width = bfr.get_f32_le();
        self._length = bfr.get_f32_le();
        self._bearing = bfr.get_f32_le();
        self._pxl = bfr.get_i16_le();
        self._encoding = bfr.get_u8();
        deserialize_bytes!(bfr, self._sonar_data);
        Ok(())
    }
}
