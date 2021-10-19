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
    /// Message Header
    pub _header: Header,
    /// Comma separated list of maneuver IDs, or the special value '.'
    /// to identify a global plan transition.
    pub _source_man: String,
    /// Target maneuver name.
    /// If it equals the special value '_done_' then plan should
    /// terminate with a success status.
    /// If it equals the special value '_error_' then the plan should
    /// terminate with an error status.
    pub _dest_man: String,
    /// Comma separated list of conditions for transition. Each
    /// condition identifier corresponds to a known predicate which is
    /// interpreted and tested internally by the vehicle.
    pub _conditions: String,
    /// Messages processed when the transition is triggered.
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._actions {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._actions {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._actions {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._actions {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._actions {
            m.set_destination_ent(dst_ent);
        }
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
