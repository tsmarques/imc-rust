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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Measure of the RSSI by a networking device.
/// Indicates the gain or loss in the signal strenght due to the transmission
/// and reception equipment and the transmission medium and distance.
#[derive(Default)]
pub struct ExtendedRSSI {
    /// Message Header
    pub _header: Header,
    /// RSSI measurement.
    pub _value: f32,
    /// Indicates the units used for the RSSI value.
    pub _units: u8,
}

impl Message for ExtendedRSSI {
    fn new() -> ExtendedRSSI {
        let msg = ExtendedRSSI {
            _header: Header::new(183),
            _value: Default::default(),
            _units: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        183
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        183
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(183);
        self._value = Default::default();
        self._units = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._value);
        bfr.put_u8(self._units);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._value = bfr.get_f32_le();
        self._units = bfr.get_u8();
        Ok(())
    }
}
