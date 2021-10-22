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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Named plan maneuver.
#[derive(Default)]
pub struct PlanManeuver {
    /// Message Header
    pub _header: Header,
    /// The maneuver ID.
    pub _maneuver_id: String,
    /// The maneuver specification.
    pub _data: Option<Box<dyn Message>>,
    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan activation.
    pub _start_actions: MessageList<Box<dyn Message>>,
    /// Contains an optionally defined 'MessageList' for actions fired
    /// on plan termination.
    pub _end_actions: MessageList<Box<dyn Message>>,
}

impl Message for PlanManeuver {
    fn new() -> PlanManeuver {
        let msg = PlanManeuver {
            _header: Header::new(552),
            _maneuver_id: Default::default(),
            _data: Default::default(),
            _start_actions: Default::default(),
            _end_actions: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        552
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        552
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        for m in &mut self._start_actions {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._end_actions {
            m.set_timestamp_secs(ts);
        }
        if let Some(m) = &mut self._data {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        for m in &mut self._start_actions {
            m.set_source(src);
        }
        for m in &mut self._end_actions {
            m.set_source(src);
        }
        if let Some(m) = &mut self._data {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        for m in &mut self._start_actions {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._end_actions {
            m.set_source_ent(src_ent);
        }
        if let Some(m) = &mut self._data {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        for m in &mut self._start_actions {
            m.set_destination(dst);
        }
        for m in &mut self._end_actions {
            m.set_destination(dst);
        }
        if let Some(m) = &mut self._data {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        for m in &mut self._start_actions {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._end_actions {
            m.set_destination_ent(dst_ent);
        }
        if let Some(m) = &mut self._data {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(552);
        self._maneuver_id = Default::default();
        self._data = Default::default();
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
        dyn_size += self._maneuver_id.len() + 2;
        inline_message_serialization_size!(dyn_size, self._data);
        message_list_serialization_size!(dyn_size, self._start_actions);
        message_list_serialization_size!(dyn_size, self._end_actions);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._maneuver_id.as_bytes());
        serialize_inline_message!(bfr, self._data);
        serialize_message_list!(bfr, self._start_actions);
        serialize_message_list!(bfr, self._end_actions);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._maneuver_id);
        self._data = deserialize_inline(bfr).ok();
        self._start_actions = deserialize_message_list(bfr)?;
        self._end_actions = deserialize_message_list(bfr)?;
        Ok(())
    }
}
