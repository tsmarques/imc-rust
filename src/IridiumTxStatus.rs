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

/// Status Code
#[allow(non_camel_case_types)]
pub enum StatusCodeEnum {
    /// Successfull transmission
    TXSTATUS_OK = 1,
    /// Error while trying to transmit message
    TXSTATUS_ERROR = 2,
    /// Message has been queued for transmission
    TXSTATUS_QUEUED = 3,
    /// Message is currently being transmitted
    TXSTATUS_TRANSMIT = 4,
    /// Message's TTL has expired. Transmition cancelled.
    TXSTATUS_EXPIRED = 5,
}

#[derive(Default)]
pub struct IridiumTxStatus {
    /// Message Header
    pub _header: Header,
    /// The request identifier used to receive transmission updates
    pub _req_id: u16,
    pub _status: u8,
    pub _text: String,
}

impl Message for IridiumTxStatus {
    fn new() -> IridiumTxStatus {
        let msg = IridiumTxStatus {
            _header: Header::new(172),
            _req_id: Default::default(),
            _status: Default::default(),
            _text: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        172
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        172
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(172);
        self._req_id = Default::default();
        self._status = Default::default();
        self._text = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._text.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u8(self._status);
        serialize_bytes!(bfr, self._text.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._status = bfr.get_u8();
        deserialize_string!(bfr, self._text);
        Ok(())
    }
}
