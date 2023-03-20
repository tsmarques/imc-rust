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

/// Group communication link assertion.
#[derive(Default, Clone)]
pub struct GroupMembershipState {
    /// Message Header
    pub _header: Header,
    /// Name of the group of systems.
    pub _group_name: String,
    /// Communication link assertion for each group member.
    /// One bit to assert each system communication link state.
    pub _links: u32,
}

impl Message for GroupMembershipState {
    fn new() -> GroupMembershipState {
        GroupMembershipState {
            _header: Header::new(180),
            _group_name: Default::default(),
            _links: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        180
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        180
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
        self._header = Header::new(180);
        self._group_name = Default::default();
        self._links = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._group_name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._group_name.as_bytes());
        bfr.put_u32_le(self._links);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._group_name);
        self._links = bfr.get_u32_le();
        Ok(())
    }
}
