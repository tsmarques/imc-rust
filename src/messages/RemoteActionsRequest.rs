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

/// operation
#[allow(non_camel_case_types)]
pub enum operationEnum {
    /// Report
    OP_REPORT = 0,
    /// Query
    OP_QUERY = 1,
}

/// This message is used as query to request for the possible remote
/// actions (operation=QUERY and the list is empty in this
/// case). The vehicle responds using the same message type
/// returning the tuplelist with the pairs: Action,Type
/// (operation=REPORT). The type of action can be Axis, Hat or
/// Button.
#[derive(Default, Clone)]
pub struct RemoteActionsRequest {
    /// Message Header
    pub _header: Header,
    /// Operation to perform.
    pub _op: u8,
    /// Example: "Propulsion=Axis,PanTilt=Hat,Lights=Button"
    pub _actions: String,
}

impl Message for RemoteActionsRequest {
    fn new() -> RemoteActionsRequest {
        RemoteActionsRequest {
            _header: Header::new(304),
            _op: Default::default(),
            _actions: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        304
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        304
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
        self._header = Header::new(304);
        self._op = Default::default();
        self._actions = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._actions.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._actions.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._actions);
        Ok(())
    }
}
