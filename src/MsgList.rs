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
use crate::MessageList;
use crate::DUNE_IMC_CONST_NULL_ID;

#[derive(Default)]
pub struct MsgList {
    /// Message Header.
    pub _header: Header,
    /// Messages.
    pub _msgs: MessageList<Box<dyn Message>>,
}

impl Message for MsgList {
    fn new() -> MsgList
    where
        Self: Sized,
    {
        let msg = MsgList {
            _header: Header::new(20),
            _msgs: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        20
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        20
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(20);
        self._msgs = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._msgs);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_message_list!(bfr, self._msgs);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._msgs = deserialize_message_list(bfr)?;
        Ok(())
    }
}
