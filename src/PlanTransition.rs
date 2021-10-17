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
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Describes a plan transition within a plan specification. A
/// transition states the vehicle conditions that must be met to
/// signal the transition, the maneuver that should be started as a
/// result, and an optional set of actions triggered by the
/// transition.
#[derive(Default)]
pub struct PlanTransition {
    /// Message Header.
    pub _header: Header,
    /// Source.
    pub _source_man: String,
    /// Destination Maneuver Name.
    pub _dest_man: String,
    /// Transition conditions.
    pub _conditions: String,
    /// Transition actions.
    pub _actions: MessageList<Box<dyn Message>>,
}

impl Message for PlanTransition {
    fn new() -> PlanTransition {
        let msg = PlanTransition {
            _header: Header::new(553),
            _source_man: Default::default(),
            _dest_man: Default::default(),
            _conditions: Default::default(),
            _actions: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        553
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        553
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(553);
        self._source_man = Default::default();
        self._dest_man = Default::default();
        self._conditions = Default::default();
        self._actions = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._source_man.len() + 2;
        dyn_size += self._dest_man.len() + 2;
        dyn_size += self._conditions.len() + 2;
        message_list_serialization_size!(dyn_size, self._actions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._source_man.as_bytes());
        serialize_bytes!(bfr, self._dest_man.as_bytes());
        serialize_bytes!(bfr, self._conditions.as_bytes());
        serialize_message_list!(bfr, self._actions);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._source_man);
        deserialize_string!(bfr, self._dest_man);
        deserialize_string!(bfr, self._conditions);
        self._actions = deserialize_message_list(bfr)?;
        Ok(())
    }
}
