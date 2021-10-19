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

/// Command
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Generate
    CMD_GENERATE = 0,
    /// Execute
    CMD_EXECUTE = 1,
}

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Request
    OP_REQUEST = 0,
    /// Error
    OP_ERROR = 1,
    /// Success
    OP_SUCCESS = 2,
}

/// This message is used to order the generation of plans based on
/// id and set of parameters.
#[derive(Default)]
pub struct PlanGeneration {
    /// Message Header
    pub _header: Header,
    pub _cmd: u8,
    pub _op: u8,
    /// The name of the plan to be generated.
    pub _plan_id: String,
    /// An optional list of parameters to be used by the plan
    /// generation module.
    pub _params: String,
}

impl Message for PlanGeneration {
    fn new() -> PlanGeneration {
        let msg = PlanGeneration {
            _header: Header::new(562),
            _cmd: Default::default(),
            _op: Default::default(),
            _plan_id: Default::default(),
            _params: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        562
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        562
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(562);
        self._cmd = Default::default();
        self._op = Default::default();
        self._plan_id = Default::default();
        self._params = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._plan_id.len() + 2;
        dyn_size += self._params.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._cmd);
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._plan_id.as_bytes());
        serialize_bytes!(bfr, self._params.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._cmd = bfr.get_u8();
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._plan_id);
        deserialize_string!(bfr, self._params);
        Ok(())
    }
}
