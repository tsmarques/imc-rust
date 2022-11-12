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
use crate::PlanManeuver::PlanManeuver;
use crate::PlanTransition::PlanTransition;
use crate::PlanVariable::PlanVariable;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Identity and description of a plan's general parameters,
/// associated with plan loading (i.e. load plan command in
/// *PlanCommand*).
/// A plan specification is defined by a plan identifier, a set of
/// maneuver specifications and a start maneuver from that set.
/// See the :ref:`PlanManeuver` message for details on maneuver
/// specification.
#[derive(Default, Clone)]
pub struct PlanSpecification {
    /// Message Header
    pub _header: Header,
    /// The plan's identifier.
    pub _plan_id: String,
    /// Verbose text description of plan.
    pub _description: String,
    /// Namespace for plan variables.
    pub _vnamespace: String,
    /// Plan variables.
    pub _variables: MessageList<PlanVariable>,
    /// Indicates the id of the starting maneuver for this plan.
    pub _start_man_id: String,
    /// List of maneuver specifications.
    pub _maneuvers: MessageList<PlanManeuver>,
    /// List of maneuver specifications.
    pub _transitions: MessageList<PlanTransition>,
    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan activation.
    pub _start_actions: MessageList<Box<dyn Message>>,
    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: MessageList<Box<dyn Message>>,
}

impl Message for PlanSpecification {
    fn new() -> PlanSpecification {
        let msg = PlanSpecification {
            _header: Header::new(551),
            _plan_id: Default::default(),
            _description: Default::default(),
            _vnamespace: Default::default(),
            _variables: Default::default(),
            _start_man_id: Default::default(),
            _maneuvers: Default::default(),
            _transitions: Default::default(),
            _start_actions: Default::default(),
            _end_actions: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        551
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        551
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._variables {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._maneuvers {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._transitions {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._start_actions {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._end_actions {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._variables {
            m.set_source(src);
        }
        for m in &mut self._maneuvers {
            m.set_source(src);
        }
        for m in &mut self._transitions {
            m.set_source(src);
        }
        for m in &mut self._start_actions {
            m.set_source(src);
        }
        for m in &mut self._end_actions {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._variables {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._maneuvers {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._transitions {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._start_actions {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._end_actions {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._variables {
            m.set_destination(dst);
        }
        for m in &mut self._maneuvers {
            m.set_destination(dst);
        }
        for m in &mut self._transitions {
            m.set_destination(dst);
        }
        for m in &mut self._start_actions {
            m.set_destination(dst);
        }
        for m in &mut self._end_actions {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._variables {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._maneuvers {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._transitions {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._start_actions {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._end_actions {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(551);
        self._plan_id = Default::default();
        self._description = Default::default();
        self._vnamespace = Default::default();
        self._variables = Default::default();
        self._start_man_id = Default::default();
        self._maneuvers = Default::default();
        self._transitions = Default::default();
        self._start_actions = Default::default();
        self._end_actions = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._description.len() + 2;
        dyn_size += self._vnamespace.len() + 2;
        message_list_serialization_size!(dyn_size, self._variables);
        dyn_size += self._start_man_id.len() + 2;
        message_list_serialization_size!(dyn_size, self._maneuvers);
        message_list_serialization_size!(dyn_size, self._transitions);
        message_list_serialization_size!(dyn_size, self._start_actions);
        message_list_serialization_size!(dyn_size, self._end_actions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._description.as_bytes());
        serialize_bytes!(bfr, self._vnamespace.as_bytes());
        serialize_message_list!(bfr, self._variables);
        serialize_bytes!(bfr, self._start_man_id.as_bytes());
        serialize_message_list!(bfr, self._maneuvers);
        serialize_message_list!(bfr, self._transitions);
        serialize_message_list!(bfr, self._start_actions);
        serialize_message_list!(bfr, self._end_actions);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._plan_id);
        deserialize_string!(bfr, self._description);
        deserialize_string!(bfr, self._vnamespace);
        self._variables = deserialize_message_list_as::<PlanVariable>(bfr)?;
        deserialize_string!(bfr, self._start_man_id);
        self._maneuvers = deserialize_message_list_as::<PlanManeuver>(bfr)?;
        self._transitions = deserialize_message_list_as::<PlanTransition>(bfr)?;
        self._start_actions = deserialize_message_list(bfr)?;
        self._end_actions = deserialize_message_list(bfr)?;
        Ok(())
    }
}
