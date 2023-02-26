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
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::PlanSpecification::PlanSpecification;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Command
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Enable
    ECTL_ENABLE = 0,
    /// Disable
    ECTL_DISABLE = 1,
    /// Start
    ECTL_START = 2,
    /// Stop
    ECTL_STOP = 3,
    /// Query
    ECTL_QUERY = 4,
    /// Set Plan
    ECTL_SET_PLAN = 5,
}

#[derive(Default, Clone)]
pub struct EmergencyControl {
    /// Message Header
    pub _header: Header,
    pub _command: u8,
    pub _plan: Option<PlanSpecification>,
}

impl Message for EmergencyControl {
    fn new() -> EmergencyControl {
        

        EmergencyControl {
            _header: Header::new(554),
            _command: Default::default(),
            _plan: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        554
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        554
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._plan {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._plan {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._plan {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._plan {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._plan {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(554);
        self._command = Default::default();
        self._plan = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._plan);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._command);
        serialize_inline_message!(bfr, self._plan);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._command = bfr.get_u8();
        self._plan = deserialize_inline_as::<PlanSpecification>(bfr).ok();
        Ok(())
    }
}
