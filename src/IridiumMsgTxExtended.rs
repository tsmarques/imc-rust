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

#[derive(Default)]
pub struct IridiumMsgTxExtended {
    /// Message Header.
    pub _header: Header,
    /// Request Identifier.
    pub _req_id: u16,
    /// Time to live.
    pub _ttl: u16,
    /// Expiration Time.
    pub _expiration: u32,
    /// Destination Identifier.
    pub _destination: String,
    /// Data.
    pub _data: Vec<u8>,
}

impl Message for IridiumMsgTxExtended {
    fn new() -> IridiumMsgTxExtended
    where
        Self: Sized,
    {
        let msg = IridiumMsgTxExtended {
            _header: Header::new(2005),
            _req_id: Default::default(),
            _ttl: Default::default(),
            _expiration: Default::default(),
            _destination: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        2005
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        2005
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(2005);
        self._req_id = Default::default();
        self._ttl = Default::default();
        self._expiration = Default::default();
        self._destination = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        8
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._destination.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._req_id);
        bfr.put_u16_le(self._ttl);
        bfr.put_u32_le(self._expiration);
        serialize_bytes!(bfr, self._destination.as_bytes());
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._req_id = bfr.get_u16_le();
        self._ttl = bfr.get_u16_le();
        self._expiration = bfr.get_u32_le();
        deserialize_string!(bfr, self._destination);
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
