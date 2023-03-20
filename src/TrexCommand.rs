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

/// Command
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Disable TREX
    OP_DISABLE = 0,
    /// Enable TREX
    OP_ENABLE = 1,
    /// Post Goal
    OP_POST_GOAL = 2,
    /// Recall Goal
    OP_RECALL_GOAL = 3,
    /// Request current plan
    OP_REQUEST_PLAN = 4,
    /// Report current plan
    OP_REPORT_PLAN = 5,
}

/// This message is used to control TREX execution
#[derive(Default, Clone)]
pub struct TrexCommand {
    /// Message Header
    pub _header: Header,
    pub _command: u8,
    /// The id of the goal, if applicable (OP == POST_GOAL || OP == RECALL_GOAL)
    pub _goal_id: String,
    /// The goal encoded as XML, if applicable (OP == POST_GOAL)
    pub _goal_xml: String,
}

impl Message for TrexCommand {
    fn new() -> TrexCommand {
        TrexCommand {
            _header: Header::new(652),
            _command: Default::default(),
            _goal_id: Default::default(),
            _goal_xml: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        652
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        652
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
        self._header = Header::new(652);
        self._command = Default::default();
        self._goal_id = Default::default();
        self._goal_xml = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._goal_id.len() + 2;
        dyn_size += self._goal_xml.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._command);
        serialize_bytes!(bfr, self._goal_id.as_bytes());
        serialize_bytes!(bfr, self._goal_xml.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._command = bfr.get_u8();
        deserialize_string!(bfr, self._goal_id);
        deserialize_string!(bfr, self._goal_xml);
        Ok(())
    }
}
