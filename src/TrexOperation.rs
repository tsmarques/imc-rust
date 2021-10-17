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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::TrexToken::TrexToken;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Operation.
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Post Token.
    OP_POST_TOKEN = 1,
    /// Post Goal.
    OP_POST_GOAL = 2,
    /// Recall Goal.
    OP_RECALL_GOAL = 3,
    /// Request current plan.
    OP_REQUEST_PLAN = 4,
    /// Report current plan.
    OP_REPORT_PLAN = 5,
}

/// This message is used to control TREX execution
#[derive(Default)]
pub struct TrexOperation {
    /// Message Header.
    pub _header: Header,
    /// Operation.
    pub _op: u8,
    /// Goal Id.
    pub _goal_id: String,
    /// Token.
    pub _token: Option<TrexToken>,
}

impl Message for TrexOperation {
    fn new() -> TrexOperation {
        let msg = TrexOperation {
            _header: Header::new(655),
            _op: Default::default(),
            _goal_id: Default::default(),
            _token: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        655
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        655
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(655);
        self._op = Default::default();
        self._goal_id = Default::default();
        self._token = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._goal_id.len() + 2;
        inline_message_serialization_size!(dyn_size, self._token);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        serialize_bytes!(bfr, self._goal_id.as_bytes());
        serialize_inline_message!(bfr, self._token);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        deserialize_string!(bfr, self._goal_id);
        self._token = deserialize_inline_as::<TrexToken>(bfr).ok();
        Ok(())
    }
}
