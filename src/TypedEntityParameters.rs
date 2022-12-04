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
use crate::MessageList;
use crate::TypedEntityParameter::TypedEntityParameter;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Request
    OP_REQUEST = 0,
    /// Reply
    OP_REPLY = 1,
}

/// This message can be used to query/report the entities and respective parameters in the system
#[derive(Default, Clone)]
pub struct TypedEntityParameters {
    /// Message Header
    pub _header: Header,
    /// Operation to perform.
    pub _op: u8,
    /// Echoes the request_id in the request
    pub _request_id: u32,
    /// Entity Label of the task that's replying to the request
    pub _entity_name: String,
    /// Contains an optionally defined List of TypedEntityParameter as a response to a TypedEntityParamaters Request.
    pub _parameters: MessageList<TypedEntityParameter>,
}

impl Message for TypedEntityParameters {
    fn new() -> TypedEntityParameters {
        

        TypedEntityParameters {
            _header: Header::new(2009),
            _op: Default::default(),
            _request_id: Default::default(),
            _entity_name: Default::default(),
            _parameters: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2009
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2009
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._parameters {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._parameters {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._parameters {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._parameters {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._parameters {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(2009);
        self._op = Default::default();
        self._request_id = Default::default();
        self._entity_name = Default::default();
        self._parameters = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._entity_name.len() + 2;
        message_list_serialization_size!(dyn_size, self._parameters);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_u32_le(self._request_id);
        serialize_bytes!(bfr, self._entity_name.as_bytes());
        serialize_message_list!(bfr, self._parameters);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._request_id = bfr.get_u32_le();
        deserialize_string!(bfr, self._entity_name);
        self._parameters = deserialize_message_list_as::<TypedEntityParameter>(bfr)?;
        Ok(())
    }
}
