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
use bytes::{Buf, BufMut};

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::LogBookEntry::LogBookEntry;
use crate::Message::*;
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Command.
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Get.
    LBC_GET = 0,
    /// Clear.
    LBC_CLEAR = 1,
    /// Get Errors.
    LBC_GET_ERR = 2,
    /// Reply.
    LBC_REPLY = 3,
}

/// Control history log.
#[derive(Default)]
pub struct LogBookControl {
    /// Message Header.
    pub _header: Header,
    /// Command.
    pub _command: u8,
    /// Timestamp.
    pub _htime: f64,
    /// Messages.
    pub _msg: MessageList<LogBookEntry>,
}

impl Message for LogBookControl {
    fn new() -> LogBookControl
    where
        Self: Sized,
    {
        let msg = LogBookControl {
            _header: Header::new(104),
            _command: Default::default(),
            _htime: Default::default(),
            _msg: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        104
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        104
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(104);
        self._command = Default::default();
        self._htime = Default::default();
        self._msg = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._msg);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._command);
        bfr.put_f64_le(self._htime);
        serialize_message_list!(bfr, self._msg);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._command = bfr.get_u8();
        self._htime = bfr.get_f64_le();
        self._msg = deserialize_message_list_as::<LogBookEntry>(bfr)?;
        Ok(())
    }
}
