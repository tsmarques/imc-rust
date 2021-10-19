//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// This message describes an entity.
#[derive(Default)]
pub struct EntityInfo {
    /// Message Header
    pub _header: Header,
    /// Entity identifier.
    pub _id: u8,
    /// Entity label or empty if the entity id is not valid.
    pub _label: String,
    /// Name of the plugin/component/subsystem associated with this
    /// entity.
    pub _component: String,
    /// Amount of time needed to properly activate the entity.
    pub _act_time: u16,
    /// Amount of time needed to properly deactivate the entity.
    pub _deact_time: u16,
}

impl Message for EntityInfo {
    fn new() -> EntityInfo {
        let msg = EntityInfo {
            _header: Header::new(3),
            _id: Default::default(),
            _label: Default::default(),
            _component: Default::default(),
            _act_time: Default::default(),
            _deact_time: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        3
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        3
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(3);
        self._id = Default::default();
        self._label = Default::default();
        self._component = Default::default();
        self._act_time = Default::default();
        self._deact_time = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._label.len() + 2;
        dyn_size += self._component.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        serialize_bytes!(bfr, self._label.as_bytes());
        serialize_bytes!(bfr, self._component.as_bytes());
        bfr.put_u16_le(self._act_time);
        bfr.put_u16_le(self._deact_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._id = bfr.get_u8();
        deserialize_string!(bfr, self._label);
        deserialize_string!(bfr, self._component);
        self._act_time = bfr.get_u16_le();
        self._deact_time = bfr.get_u16_le();
        Ok(())
    }
}
