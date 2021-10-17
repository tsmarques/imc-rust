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
pub struct IridiumMsgRx {
    /// Message Header.
    pub _header: Header,
    /// Origin Identifier.
    pub _origin: String,
    /// Timestamp.
    pub _htime: f64,
    /// Latitude Reference.
    pub _lat: f64,
    /// Longitude Reference.
    pub _lon: f64,
    /// Data.
    pub _data: Vec<u8>,
}

impl Message for IridiumMsgRx {
    fn new() -> IridiumMsgRx
    where
        Self: Sized,
    {
        let msg = IridiumMsgRx {
            _header: Header::new(170),
            _origin: Default::default(),
            _htime: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _data: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        170
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        170
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(170);
        self._origin = Default::default();
        self._htime = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._data = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._origin.len() + 2;
        dyn_size += self._data.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._origin.as_bytes());
        bfr.put_f64_le(self._htime);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        serialize_bytes!(bfr, self._data.as_slice());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._origin);
        self._htime = bfr.get_f64_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        deserialize_bytes!(bfr, self._data);
        Ok(())
    }
}
