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
use crate::Message::*;
use crate::MessageList;
use crate::TrexAttribute::TrexAttribute;
use crate::DUNE_IMC_CONST_NULL_ID;

#[derive(Default)]
pub struct TrexToken {
    /// Message Header.
    pub _header: Header,
    /// Timeline.
    pub _timeline: String,
    /// Predicate.
    pub _predicate: String,
    /// Attributes.
    pub _attributes: MessageList<TrexAttribute>,
}

impl Message for TrexToken {
    fn new() -> TrexToken
    where
        Self: Sized,
    {
        let msg = TrexToken {
            _header: Header::new(657),
            _timeline: Default::default(),
            _predicate: Default::default(),
            _attributes: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        657
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        657
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(657);
        self._timeline = Default::default();
        self._predicate = Default::default();
        self._attributes = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._timeline.len() + 2;
        dyn_size += self._predicate.len() + 2;
        message_list_serialization_size!(dyn_size, self._attributes);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._timeline.as_bytes());
        serialize_bytes!(bfr, self._predicate.as_bytes());
        serialize_message_list!(bfr, self._attributes);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._timeline);
        deserialize_string!(bfr, self._predicate);
        self._attributes = deserialize_message_list_as::<TrexAttribute>(bfr)?;
        Ok(())
    }
}
