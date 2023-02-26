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

/// Group List Action
#[allow(non_camel_case_types)]
pub enum GroupListActionEnum {
    /// Disband
    OP_Dis = 0,
    /// Set
    OP_Set = 1,
    /// Request
    OP_Req = 2,
    /// Change
    OP_Chg = 3,
    /// Report
    OP_Rep = 4,
    /// Force
    OP_Frc = 5,
}

/// Group of systems configuration.
#[derive(Default, Clone)]
pub struct SystemGroup {
    /// Message Header
    pub _header: Header,
    /// Name of the group of systems.
    pub _GroupName: String,
    /// Actions on the group list.
    pub _Action: u8,
    /// List of names of system in the group, separated by commas.
    pub _GroupList: String,
}

impl Message for SystemGroup {
    fn new() -> SystemGroup {
        

        SystemGroup {
            _header: Header::new(181),
            _GroupName: Default::default(),
            _Action: Default::default(),
            _GroupList: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        181
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        181
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
        self._header = Header::new(181);
        self._GroupName = Default::default();
        self._Action = Default::default();
        self._GroupList = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._GroupName.len() + 2;
        dyn_size += self._GroupList.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._GroupName.as_bytes());
        bfr.put_u8(self._Action);
        serialize_bytes!(bfr, self._GroupList.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._GroupName);
        self._Action = bfr.get_u8();
        deserialize_string!(bfr, self._GroupList);
        Ok(())
    }
}
