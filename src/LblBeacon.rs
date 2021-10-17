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

/// Position and configuration of an LBL transponder (beacon).
#[derive(Default)]
pub struct LblBeacon {
    /// Message Header.
    pub _header: Header,
    /// Beacon Name.
    pub _beacon: String,
    /// Latitude WGS-84.
    pub _lat: f64,
    /// Longitude WGS-84.
    pub _lon: f64,
    /// Depth.
    pub _depth: f32,
    /// Interrogation channel.
    pub _query_channel: u8,
    /// Reply channel.
    pub _reply_channel: u8,
    /// Transponder delay.
    pub _transponder_delay: u8,
}

impl Message for LblBeacon {
    fn new() -> LblBeacon
    where
        Self: Sized,
    {
        let msg = LblBeacon {
            _header: Header::new(202),
            _beacon: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _depth: Default::default(),
            _query_channel: Default::default(),
            _reply_channel: Default::default(),
            _transponder_delay: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        202
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
        202
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(202);
        self._beacon = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._depth = Default::default();
        self._query_channel = Default::default();
        self._reply_channel = Default::default();
        self._transponder_delay = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        23
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._beacon.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._beacon.as_bytes());
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._depth);
        bfr.put_u8(self._query_channel);
        bfr.put_u8(self._reply_channel);
        bfr.put_u8(self._transponder_delay);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._beacon);
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._depth = bfr.get_f32_le();
        self._query_channel = bfr.get_u8();
        self._reply_channel = bfr.get_u8();
        self._transponder_delay = bfr.get_u8();
        Ok(())
    }
}
