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
use crate::Command::Command;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// State.
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Waiting for first command.
    FC_WAIT = 1,
    /// Moving towards received command.
    FC_MOVING = 2,
    /// Speed command is zero.
    FC_STOPPED = 3,
    /// Command is out of safe bounds.
    FC_BAD_COMMAND = 4,
    /// Controlling system timed out.
    FC_TIMEOUT = 5,
}

#[derive(Default)]
pub struct FollowCommandState {
    /// Message Header.
    pub _header: Header,
    /// Controlling Source.
    pub _control_src: u16,
    /// Controlling Entity.
    pub _control_ent: u8,
    /// Command.
    pub _command: Option<Command>,
    /// State.
    pub _state: u8,
}

impl Message for FollowCommandState {
    fn new() -> FollowCommandState {
        let msg = FollowCommandState {
            _header: Header::new(498),
            _control_src: Default::default(),
            _control_ent: Default::default(),
            _command: Default::default(),
            _state: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        498
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        498
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(498);
        self._control_src = Default::default();
        self._control_ent = Default::default();
        self._command = Default::default();
        self._state = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        4
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._command);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._control_src);
        bfr.put_u8(self._control_ent);
        serialize_inline_message!(bfr, self._command);
        bfr.put_u8(self._state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._control_src = bfr.get_u16_le();
        self._control_ent = bfr.get_u8();
        self._command = deserialize_inline_as::<Command>(bfr).ok();
        self._state = bfr.get_u8();
        Ok(())
    }
}
