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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Attribute type
#[allow(non_camel_case_types)]
pub enum AttributetypeEnum {
    /// Boolean Domain
    TYPE_BOOL = 1,
    /// Integer Domain
    TYPE_INT = 2,
    /// Float Domain
    TYPE_FLOAT = 3,
    /// String Domain
    TYPE_STRING = 4,
    /// Enumerated Domain
    TYPE_ENUM = 5,
}

#[derive(Default)]
pub struct TrexAttribute {
    /// Message Header
    pub _header: Header,
    /// Name of this attribute.
    pub _name: String,
    pub _attr_type: u8,
    /// Lower bound of this interval. Empty text means no bound.
    pub _min: String,
    /// Upper bound of this interval. Empty text means no bound.
    pub _max: String,
}

impl Message for TrexAttribute {
    fn new() -> TrexAttribute {
        let msg = TrexAttribute {
            _header: Header::new(656),
            _name: Default::default(),
            _attr_type: Default::default(),
            _min: Default::default(),
            _max: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        656
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        656
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(656);
        self._name = Default::default();
        self._attr_type = Default::default();
        self._min = Default::default();
        self._max = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;
        dyn_size += self._min.len() + 2;
        dyn_size += self._max.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._attr_type);
        serialize_bytes!(bfr, self._min.as_bytes());
        serialize_bytes!(bfr, self._max.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);
        self._attr_type = bfr.get_u8();
        deserialize_string!(bfr, self._min);
        deserialize_string!(bfr, self._max);
        Ok(())
    }
}
