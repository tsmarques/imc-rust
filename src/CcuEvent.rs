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
use crate::DUNE_IMC_CONST_NULL_ID;

/// Event Type
#[allow(non_camel_case_types)]
pub enum EventTypeEnum {
    /// Log Book Entry Added
    EVT_LOG_ENTRY = 1,
    /// Plan Added
    EVT_PLAN_ADDED = 2,
    /// Plan Removed
    EVT_PLAN_REMOVED = 3,
    /// Plan Changed
    EVT_PLAN_CHANGED = 4,
    /// Map feature added
    EVT_MAP_FEATURE_ADDED = 5,
    /// Map feature removed
    EVT_MAP_FEATURE_REMOVED = 6,
    /// Map feature changed
    EVT_MAP_FEATURE_CHANGED = 7,
    /// The sender is now teleoperating the vehicle
    EVT_TELEOPERATION_STARTED = 8,
    /// The sender stopped teleoperating the vehicle
    EVT_TELEOPERATION_ENDED = 9,
}

/// This message is used to signal events among running CCUs.
#[derive(Default)]
pub struct CcuEvent {
    /// Message Header
    pub _header: Header,
    pub _type: u8,
    pub _id: String,
    pub _arg: Option<Box<dyn Message>>,
}

impl Message for CcuEvent {
    fn new() -> CcuEvent {
        let msg = CcuEvent {
            _header: Header::new(606),
            _type: Default::default(),
            _id: Default::default(),
            _arg: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        606
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        606
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._arg {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._arg {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._arg {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._arg {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._arg {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(606);
        self._type = Default::default();
        self._id = Default::default();
        self._arg = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._id.len() + 2;
        inline_message_serialization_size!(dyn_size, self._arg);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        serialize_bytes!(bfr, self._id.as_bytes());
        serialize_inline_message!(bfr, self._arg);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        deserialize_string!(bfr, self._id);
        self._arg = deserialize_inline(bfr).ok();
        Ok(())
    }
}
