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

/// Presence of DMS (Dimethyl Sulphide).
/// If the value of the channel is greater than zero, it means DMS was detected.
#[derive(Default, Clone)]
pub struct DmsDetection {
    /// Message Header
    pub _header: Header,
    pub _ch01: f32,
    pub _ch02: f32,
    pub _ch03: f32,
    pub _ch04: f32,
    pub _ch05: f32,
    pub _ch06: f32,
    pub _ch07: f32,
    pub _ch08: f32,
    pub _ch09: f32,
    pub _ch10: f32,
    pub _ch11: f32,
    pub _ch12: f32,
    pub _ch13: f32,
    pub _ch14: f32,
    pub _ch15: f32,
    pub _ch16: f32,
}

impl Message for DmsDetection {
    fn new() -> DmsDetection {
        DmsDetection {
            _header: Header::new(908),
            _ch01: Default::default(),
            _ch02: Default::default(),
            _ch03: Default::default(),
            _ch04: Default::default(),
            _ch05: Default::default(),
            _ch06: Default::default(),
            _ch07: Default::default(),
            _ch08: Default::default(),
            _ch09: Default::default(),
            _ch10: Default::default(),
            _ch11: Default::default(),
            _ch12: Default::default(),
            _ch13: Default::default(),
            _ch14: Default::default(),
            _ch15: Default::default(),
            _ch16: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        908
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        908
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
        self._header = Header::new(908);
        self._ch01 = Default::default();
        self._ch02 = Default::default();
        self._ch03 = Default::default();
        self._ch04 = Default::default();
        self._ch05 = Default::default();
        self._ch06 = Default::default();
        self._ch07 = Default::default();
        self._ch08 = Default::default();
        self._ch09 = Default::default();
        self._ch10 = Default::default();
        self._ch11 = Default::default();
        self._ch12 = Default::default();
        self._ch13 = Default::default();
        self._ch14 = Default::default();
        self._ch15 = Default::default();
        self._ch16 = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        64
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._ch01);
        bfr.put_f32_le(self._ch02);
        bfr.put_f32_le(self._ch03);
        bfr.put_f32_le(self._ch04);
        bfr.put_f32_le(self._ch05);
        bfr.put_f32_le(self._ch06);
        bfr.put_f32_le(self._ch07);
        bfr.put_f32_le(self._ch08);
        bfr.put_f32_le(self._ch09);
        bfr.put_f32_le(self._ch10);
        bfr.put_f32_le(self._ch11);
        bfr.put_f32_le(self._ch12);
        bfr.put_f32_le(self._ch13);
        bfr.put_f32_le(self._ch14);
        bfr.put_f32_le(self._ch15);
        bfr.put_f32_le(self._ch16);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._ch01 = bfr.get_f32_le();
        self._ch02 = bfr.get_f32_le();
        self._ch03 = bfr.get_f32_le();
        self._ch04 = bfr.get_f32_le();
        self._ch05 = bfr.get_f32_le();
        self._ch06 = bfr.get_f32_le();
        self._ch07 = bfr.get_f32_le();
        self._ch08 = bfr.get_f32_le();
        self._ch09 = bfr.get_f32_le();
        self._ch10 = bfr.get_f32_le();
        self._ch11 = bfr.get_f32_le();
        self._ch12 = bfr.get_f32_le();
        self._ch13 = bfr.get_f32_le();
        self._ch14 = bfr.get_f32_le();
        self._ch15 = bfr.get_f32_le();
        self._ch16 = bfr.get_f32_le();
        Ok(())
    }
}
