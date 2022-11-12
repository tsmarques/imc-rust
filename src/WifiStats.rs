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

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// This message is used to log wifi connection statistics, heavily influenced by the stats available in ubiquiti radios.
#[derive(Default, Clone)]
pub struct WifiStats {
    /// Message Header
    pub _header: Header,
    /// MAC address of the associated radio.
    pub _mac: String,
    /// Last IP address of the associated radio.
    pub _ip: String,
    /// Client Connection Quality indicator
    pub _ccq: u8,
    /// Noise Floor (measure of the signal created from the sum of all the noise sources
    /// and unwanted signals within a measurement system, where noise is defined as any signal other
    /// than the one being monitored).
    pub _noise_floor: i16,
    /// Measure of the signal of the associated radio.
    pub _signal: i16,
    /// Received Signal Strength Indicator, in arbitraty units. The bigger the RSSI, the better
    /// the connection quality.
    pub _rssi: u16,
    /// Reception data rate for the associated radio, -1 if not available.
    pub _rx_rate: i16,
    /// Transmission data rate for the associated radio, -1 if not available.
    pub _tx_rate: i16,
    /// Latency of transmission to the associated radio, -1 if not available.
    pub _tx_latency: i16,
    /// Power of transmission to the associated radio, -1 if not available.
    pub _tx_power: i16,
    /// Amount of bytes already received from the associated radio.
    pub _rx_count: u32,
    /// Amount of bytes already transmitted to the associated radio.
    pub _tx_count: u32,
    /// Distance for the associated radio, -1 if not available.
    pub _distance: i16,
}

impl Message for WifiStats {
    fn new() -> WifiStats {
        let msg = WifiStats {
            _header: Header::new(2011),
            _mac: Default::default(),
            _ip: Default::default(),
            _ccq: Default::default(),
            _noise_floor: Default::default(),
            _signal: Default::default(),
            _rssi: Default::default(),
            _rx_rate: Default::default(),
            _tx_rate: Default::default(),
            _tx_latency: Default::default(),
            _tx_power: Default::default(),
            _rx_count: Default::default(),
            _tx_count: Default::default(),
            _distance: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2011
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2011
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(2011);
        self._mac = Default::default();
        self._ip = Default::default();
        self._ccq = Default::default();
        self._noise_floor = Default::default();
        self._signal = Default::default();
        self._rssi = Default::default();
        self._rx_rate = Default::default();
        self._tx_rate = Default::default();
        self._tx_latency = Default::default();
        self._tx_power = Default::default();
        self._rx_count = Default::default();
        self._tx_count = Default::default();
        self._distance = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        25
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._mac.len() + 2;
        dyn_size += self._ip.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._mac.as_bytes());
        serialize_bytes!(bfr, self._ip.as_bytes());
        bfr.put_u8(self._ccq);
        bfr.put_i16_le(self._noise_floor);
        bfr.put_i16_le(self._signal);
        bfr.put_u16_le(self._rssi);
        bfr.put_i16_le(self._rx_rate);
        bfr.put_i16_le(self._tx_rate);
        bfr.put_i16_le(self._tx_latency);
        bfr.put_i16_le(self._tx_power);
        bfr.put_u32_le(self._rx_count);
        bfr.put_u32_le(self._tx_count);
        bfr.put_i16_le(self._distance);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._mac);
        deserialize_string!(bfr, self._ip);
        self._ccq = bfr.get_u8();
        self._noise_floor = bfr.get_i16_le();
        self._signal = bfr.get_i16_le();
        self._rssi = bfr.get_u16_le();
        self._rx_rate = bfr.get_i16_le();
        self._tx_rate = bfr.get_i16_le();
        self._tx_latency = bfr.get_i16_le();
        self._tx_power = bfr.get_i16_le();
        self._rx_count = bfr.get_u32_le();
        self._tx_count = bfr.get_u32_le();
        self._distance = bfr.get_i16_le();
        Ok(())
    }
}
