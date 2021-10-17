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

/// Value.
#[allow(non_camel_case_types)]
pub enum ValueEnum {
    /// Transmission Completed.
    UTS_DONE = 0,
    /// Transmission Failed.
    UTS_FAILED = 1,
    /// Transmission Canceled.
    UTS_CANCELED = 2,
    /// Modem is busy.
    UTS_BUSY = 3,
    /// Invalid address.
    UTS_INV_ADDR = 4,
    /// In Progress.
    UTS_IP = 5,
    /// Unsupported operation.
    UTS_UNSUPPORTED = 6,
    /// Invalid transmission size.
    UTS_INV_SIZE = 7,
    /// Not transducer.
    UTS_NOT_TRANSDUCER = 8,
}

/// This message shall be used by acoustic modem drivers to send updates
/// on the transmission status of data frames.
#[derive(Default)]
pub struct UamTxStatus {
    /// Message Header.
    pub _header: Header,
    /// Sequence Id.
    pub _seq: u16,
    /// Value.
    pub _value: u8,
    /// Error Message.
    pub _error: String,
}

impl Message for UamTxStatus {
    fn new() -> UamTxStatus
    where
        Self: Sized,
    {
        let msg = UamTxStatus {
            _header: Header::new(816),
            _seq: Default::default(),
            _value: Default::default(),
            _error: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        816
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        816
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(816);
        self._seq = Default::default();
        self._value = Default::default();
        self._error = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._seq);
        bfr.put_u8(self._value);
        serialize_bytes!(bfr, self._error.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._seq = bfr.get_u16_le();
        self._value = bfr.get_u8();
        deserialize_string!(bfr, self._error);
        Ok(())
    }
}
