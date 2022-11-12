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
use crate::MessageList;
use crate::ValuesIf::ValuesIf;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Boolean Domain
    TYPE_BOOL = 1,
    /// Integer Domain
    TYPE_INT = 2,
    /// Float Domain
    TYPE_FLOAT = 3,
    /// String Domain
    TYPE_STRING = 4,
    /// List of Booleans
    TYPE_LIST_BOOL = 5,
    /// List of Integers
    TYPE_LIST_INT = 6,
    /// List of Floats
    TYPE_LIST_FLOAT = 7,
    /// List of Strings
    TYPE_LIST_STRING = 8,
}

/// Visibility
#[allow(non_camel_case_types)]
pub enum VisibilityEnum {
    /// User
    VISIBILITY_user = 0,
    /// Developer
    VISIBILITY_developer = 1,
}

/// Scope
#[allow(non_camel_case_types)]
pub enum ScopeEnum {
    /// Global
    SCOPE_global = 0,
    /// Idle
    SCOPE_idle = 1,
    /// Plan
    SCOPE_plan = 2,
    /// Maneuver
    SCOPE_maneuver = 3,
}

/// Entity parameter with all the data that defines an entity parameter.
#[derive(Default, Clone)]
pub struct TypedEntityParameter {
    /// Message Header
    pub _header: Header,
    /// Name of the parameter.
    pub _name: String,
    pub _type: u8,
    /// Default value of the parameter.
    pub _default_value: String,
    /// The units of the field, if applicable
    pub _units: String,
    /// Description of the parameter
    pub _description: String,
    /// Comma-separated list of possible values
    pub _values: String,
    /// Optional. Min value of the parameter
    pub _min_value: f32,
    /// Optional. Max value of the parameter
    pub _max_value: f32,
    /// When the parameter is a list, list_min_size indicates the minimum size of the list
    pub _list_min_size: u32,
    /// When the parameter is a list, list_max_size indicates the maximum size of the list
    pub _list_max_size: u32,
    /// A list of ValuesIf messages
    pub _values_if_list: MessageList<ValuesIf>,
    pub _visibility: u8,
    pub _scope: u8,
}

impl Message for TypedEntityParameter {
    fn new() -> TypedEntityParameter {
        let msg = TypedEntityParameter {
            _header: Header::new(2008),
            _name: Default::default(),
            _type: Default::default(),
            _default_value: Default::default(),
            _units: Default::default(),
            _description: Default::default(),
            _values: Default::default(),
            _min_value: Default::default(),
            _max_value: Default::default(),
            _list_min_size: Default::default(),
            _list_max_size: Default::default(),
            _values_if_list: Default::default(),
            _visibility: Default::default(),
            _scope: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2008
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2008
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._values_if_list {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._values_if_list {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._values_if_list {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._values_if_list {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._values_if_list {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(2008);
        self._name = Default::default();
        self._type = Default::default();
        self._default_value = Default::default();
        self._units = Default::default();
        self._description = Default::default();
        self._values = Default::default();
        self._min_value = Default::default();
        self._max_value = Default::default();
        self._list_min_size = Default::default();
        self._list_max_size = Default::default();
        self._values_if_list = Default::default();
        self._visibility = Default::default();
        self._scope = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        19
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._name.len() + 2;
        dyn_size += self._default_value.len() + 2;
        dyn_size += self._units.len() + 2;
        dyn_size += self._description.len() + 2;
        dyn_size += self._values.len() + 2;
        message_list_serialization_size!(dyn_size, self._values_if_list);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._name.as_bytes());
        bfr.put_u8(self._type);
        serialize_bytes!(bfr, self._default_value.as_bytes());
        serialize_bytes!(bfr, self._units.as_bytes());
        serialize_bytes!(bfr, self._description.as_bytes());
        serialize_bytes!(bfr, self._values.as_bytes());
        bfr.put_f32_le(self._min_value);
        bfr.put_f32_le(self._max_value);
        bfr.put_u32_le(self._list_min_size);
        bfr.put_u32_le(self._list_max_size);
        serialize_message_list!(bfr, self._values_if_list);
        bfr.put_u8(self._visibility);
        bfr.put_u8(self._scope);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._name);
        self._type = bfr.get_u8();
        deserialize_string!(bfr, self._default_value);
        deserialize_string!(bfr, self._units);
        deserialize_string!(bfr, self._description);
        deserialize_string!(bfr, self._values);
        self._min_value = bfr.get_f32_le();
        self._max_value = bfr.get_f32_le();
        self._list_min_size = bfr.get_u32_le();
        self._list_max_size = bfr.get_u32_le();
        self._values_if_list = deserialize_message_list_as::<ValuesIf>(bfr)?;
        self._visibility = bfr.get_u8();
        self._scope = bfr.get_u8();
        Ok(())
    }
}
