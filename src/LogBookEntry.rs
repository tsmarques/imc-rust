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

/// Type.
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Information.
    LBET_INFO = 0,
    /// Warning.
    LBET_WARNING = 1,
    /// Error.
    LBET_ERROR = 2,
    /// Critical.
    LBET_CRITICAL = 3,
    /// Debug.
    LBET_DEBUG = 4,
}

/// Human readable message reporting an event of interest.
#[derive(Default)]
pub struct LogBookEntry {
    /// Message Header.
    pub _header: Header,
    /// Type.
    pub _type: u8,
    /// Timestamp.
    pub _htime: f64,
    /// Context.
    pub _context: String,
    /// Text.
    pub _text: String,
}

impl Message for LogBookEntry {
    fn new() -> LogBookEntry {
        let msg = LogBookEntry {
            _header: Header::new(103),
            _type: Default::default(),
            _htime: Default::default(),
            _context: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        103
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        103
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(103);
        self._type = Default::default();
        self._htime = Default::default();
        self._context = Default::default();
        self._text = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._context.len() + 2;
        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_f64_le(self._htime);
        serialize_bytes!(bfr, self._context.as_bytes());
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._htime = bfr.get_f64_le();
        deserialize_string!(bfr, self._context);
        deserialize_string!(bfr, self._text);
        Ok(())
    }
}
