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
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type.
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request.
    PC_REQUEST = 0,
    /// Reply -- Success.
    PC_SUCCESS = 1,
    /// Reply -- Failure.
    PC_FAILURE = 2,
    /// Reply -- In Progress.
    PC_IN_PROGRESS = 3,
}

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Start Plan.
    PC_START = 0,
    /// Stop Plan.
    PC_STOP = 1,
    /// Load Plan.
    PC_LOAD = 2,
    /// Get Plan.
    PC_GET = 3,
}

/// Flags.
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Calibrate Vehicle.
    pub const FLG_CALIBRATE: u32 = 0x0001;
    /// Ignore Errors.
    pub const FLG_IGNORE_ERRORS: u32 = 0x0002;
}

/// Plan control request/reply.
#[derive(Default)]
pub struct PlanControl {
    /// Message Header.
    pub _header: Header,
    /// Type.
    pub _type: u8,
    /// Operation.
    pub _op: u8,
    /// Request ID.
    pub _request_id: u16,
    /// Plan Identifier.
    pub _plan_id: String,
    /// Flags.
    pub _flags: u16,
    /// Request/Reply Argument.
    pub _arg: Option<Box<dyn Message>>,
    /// Complementary Info.
    pub _info: String,
}

impl Message for PlanControl {
    fn new() -> PlanControl {
        let msg = PlanControl {
            _header: Header::new(559),
            _type: Default::default(),
            _op: Default::default(),
            _request_id: Default::default(),
            _plan_id: Default::default(),
            _flags: Default::default(),
            _arg: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        559
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        559
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
        self._header = Header::new(559);
        self._type = Default::default();
        self._op = Default::default();
        self._request_id = Default::default();
        self._plan_id = Default::default();
        self._flags = Default::default();
        self._arg = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;
        inline_message_serialization_size!(dyn_size, self._arg);
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._op);
        bfr.put_u16_le(self._request_id);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        bfr.put_u16_le(self._flags);
        serialize_inline_message!(bfr, self._arg);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._op = bfr.get_u8();
        self._request_id = bfr.get_u16_le();
        deserialize_string!(bfr, self._plan_id);
        self._flags = bfr.get_u16_le();
        self._arg = deserialize_inline(bfr).ok();
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
