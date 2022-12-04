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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Request to send SMS.
#[derive(Default, Clone)]
pub struct SmsTx {
    /// Message Header
    pub _header: Header,
    /// Sequence number.
    pub _seq: u32,
    /// Number or name of the recipient.
    pub _destination: String,
    /// Timeout for sending message.
    pub _timeout: u16,
    /// Message data.
    pub _data: Vec<u8>,
}

impl Message for SmsTx {
    fn new() -> SmsTx {
        

        SmsTx {
            _header: Header::new(157),
            _seq: Default::default(),
            _destination: Default::default(),
            _timeout: Default::default(),
            _data: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        157
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        157
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(157);
        self._seq = Default::default();
        self._destination = Default::default();
        self._timeout = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._destination.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._seq);
        serialize_bytes!(bfr, self._destination.as_bytes());
        bfr.put_u16_le(self._timeout);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u32_le();
        deserialize_string!(bfr, self._destination);
        self._timeout = bfr.get_u16_le();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
