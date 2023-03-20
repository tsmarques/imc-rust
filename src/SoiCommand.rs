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
use crate::SoiPlan::SoiPlan;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request
    SOITYPE_REQUEST = 1,
    /// Success
    SOITYPE_SUCCESS = 2,
    /// Error
    SOITYPE_ERROR = 3,
}

/// Command
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Execute Plan
    SOICMD_EXEC = 1,
    /// Stop Execution
    SOICMD_STOP = 2,
    /// Set Parameters
    SOICMD_SET_PARAMS = 3,
    /// Get Parameters
    SOICMD_GET_PARAMS = 4,
    /// Get Plan
    SOICMD_GET_PLAN = 5,
    /// Resume Execution
    SOICMD_RESUME = 6,
}

#[derive(Default, Clone)]
pub struct SoiCommand {
    /// Message Header
    pub _header: Header,
    pub _type: u8,
    pub _command: u8,
    pub _settings: String,
    pub _plan: Option<SoiPlan>,
    pub _info: String,
}

impl Message for SoiCommand {
    fn new() -> SoiCommand {
        SoiCommand {
            _header: Header::new(852),
            _type: Default::default(),
            _command: Default::default(),
            _settings: Default::default(),
            _plan: Default::default(),
            _info: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        852
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        852
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts;
        if let Some(m) = &mut self._plan {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        if let Some(m) = &mut self._plan {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        if let Some(m) = &mut self._plan {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        if let Some(m) = &mut self._plan {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._plan {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(852);
        self._type = Default::default();
        self._command = Default::default();
        self._settings = Default::default();
        self._plan = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._settings.len() + 2;
        inline_message_serialization_size!(dyn_size, self._plan);
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._command);
        serialize_bytes!(bfr, self._settings.as_bytes());
        serialize_inline_message!(bfr, self._plan);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._command = bfr.get_u8();
        deserialize_string!(bfr, self._settings);
        self._plan = deserialize_inline_as::<SoiPlan>(bfr).ok();
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
