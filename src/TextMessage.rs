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

/// A text message has been received.
#[derive(Default)]
pub struct TextMessage {
    /// Message Header.
    pub _header: Header,
    /// Origin.
    pub _origin: String,
    /// Text.
    pub _text: String,
}

impl Message for TextMessage {
    fn new() -> TextMessage
    where
        Self: Sized,
    {
        let msg = TextMessage {
            _header: Header::new(160),
            _origin: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        160
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        160
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(160);
        self._origin = Default::default();
        self._text = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        0
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._origin.len() + 2;
        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._origin.as_bytes());
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._origin);
        deserialize_string!(bfr, self._text);
        Ok(())
    }
}
