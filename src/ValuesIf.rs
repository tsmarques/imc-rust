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

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// This message is used to describe the ValuesIf content of a TypedEntityParameter.
#[derive(Default, Clone)]
pub struct ValuesIf {
    /// Message Header
    pub _header: Header,
    /// Name of parameter to compare
    pub _param: String,
    /// Value to compare
    pub _value: String,
    /// List of possible values if param=value
    pub _values: String,
}

impl Message for ValuesIf {
    fn new() -> ValuesIf {
        let msg = ValuesIf {
            _header: Header::new(2014),
            _param: Default::default(),
            _value: Default::default(),
            _values: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2014
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2014
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
        self._header = Header::new(2014);
        self._param = Default::default();
        self._value = Default::default();
        self._values = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._param.len() + 2;
        dyn_size += self._value.len() + 2;
        dyn_size += self._values.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._param.as_bytes());
        serialize_bytes!(bfr, self._value.as_bytes());
        serialize_bytes!(bfr, self._values.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._param);
        deserialize_string!(bfr, self._value);
        deserialize_string!(bfr, self._values);
        Ok(())
    }
}
