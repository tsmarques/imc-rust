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

/// This message is used to log wifi networks in the surroundings.
#[derive(Default, Clone)]
pub struct WifiNetwork {
    /// Message Header
    pub _header: Header,
    /// Extended Service Set Identifier of the network
    pub _essid: String,
    /// MAC Address of the network.
    pub _mac: String,
    pub _signal: i16,
    pub _noise: i16,
    pub _ccq: i8,
    pub _channel: u8,
    pub _freq: f32,
    pub _security: String,
}

impl Message for WifiNetwork {
    fn new() -> WifiNetwork {
        

        WifiNetwork {
            _header: Header::new(2012),
            _essid: Default::default(),
            _mac: Default::default(),
            _signal: Default::default(),
            _noise: Default::default(),
            _ccq: Default::default(),
            _channel: Default::default(),
            _freq: Default::default(),
            _security: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2012
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2012
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
        self._header = Header::new(2012);
        self._essid = Default::default();
        self._mac = Default::default();
        self._signal = Default::default();
        self._noise = Default::default();
        self._ccq = Default::default();
        self._channel = Default::default();
        self._freq = Default::default();
        self._security = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        10
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._essid.len() + 2;
        dyn_size += self._mac.len() + 2;
        dyn_size += self._security.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._essid.as_bytes());
        serialize_bytes!(bfr, self._mac.as_bytes());
        bfr.put_i16_le(self._signal);
        bfr.put_i16_le(self._noise);
        bfr.put_i8(self._ccq);
        bfr.put_u8(self._channel);
        bfr.put_f32_le(self._freq);
        serialize_bytes!(bfr, self._security.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._essid);
        deserialize_string!(bfr, self._mac);
        self._signal = bfr.get_i16_le();
        self._noise = bfr.get_i16_le();
        self._ccq = bfr.get_i8();
        self._channel = bfr.get_u8();
        self._freq = bfr.get_f32_le();
        deserialize_string!(bfr, self._security);
        Ok(())
    }
}
