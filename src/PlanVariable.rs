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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Boolean
    PVT_BOOLEAN = 0,
    /// Number
    PVT_NUMBER = 1,
    /// Text
    PVT_TEXT = 2,
    /// Message
    PVT_MESSAGE = 3,
}

/// Access Type
#[allow(non_camel_case_types)]
pub enum AccessTypeEnum {
    /// Input
    PVA_INPUT = 0,
    /// Output
    PVA_OUTPUT = 1,
    /// Local
    PVA_LOCAL = 2,
}

/// A plan variable.
#[derive(Default, Clone)]
pub struct PlanVariable {
    /// Message Header
    pub _header: Header,
    pub _name: String,
    pub _value: String,
    pub _type: u8,
    pub _access: u8,
}

impl Message for PlanVariable {
    fn new() -> PlanVariable {
        let msg = PlanVariable {
            _header: Header::new(561),
            _name: Default::default(),
            _value: Default::default(),
            _type: Default::default(),
            _access: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        561
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        561
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(561);
        self._name = Default::default();
        self._value = Default::default();
        self._type = Default::default();
        self._access = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;
        dyn_size += self._value.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        serialize_bytes!(bfr, self._value.as_bytes());
        bfr.put_u8(self._type);
        bfr.put_u8(self._access);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);
        deserialize_string!(bfr, self._value);
        self._type = bfr.get_u8();
        self._access = bfr.get_u8();
        Ok(())
    }
}
