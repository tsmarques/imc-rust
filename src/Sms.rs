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

/// Send a SMS message.
#[derive(Default)]
pub struct Sms {
    /// Message Header.
    pub _header: Header,
    /// Number.
    pub _number: String,
    /// Timeout.
    pub _timeout: u16,
    /// Contents.
    pub _contents: String,
}

impl Message for Sms {
    fn new() -> Sms
    where
        Self: Sized,
    {
        let msg = Sms {
            _header: Header::new(156),
            _number: Default::default(),
            _timeout: Default::default(),
            _contents: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        156
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        156
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(156);
        self._number = Default::default();
        self._timeout = Default::default();
        self._contents = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        2
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._number.len() + 2;
        dyn_size += self._contents.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._number.as_bytes());
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._contents.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._number);
        self._timeout = bfr.get_u16_le();
        deserialize_string!(bfr, self._contents);
        Ok(())
    }
}
