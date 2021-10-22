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

/// Information regarding a sent/received Sonar pulse.
#[derive(Default)]
pub struct SonarPulse {
    /// Message Header
    pub _header: Header,
    /// Frequency of the sent/received sonar pulse.
    pub _frequency: i32,
    /// Pulse Length of the sonar pulse.
    pub _pulse_length: i32,
    /// Time Delay of the sonar pulse.
    pub _time_delay: i32,
    /// Doppler shift added to the sonar pulse in retransmission
    pub _simulated_speed: i32,
}

impl Message for SonarPulse {
    fn new() -> SonarPulse {
        let msg = SonarPulse {
            _header: Header::new(2006),
            _frequency: Default::default(),
            _pulse_length: Default::default(),
            _time_delay: Default::default(),
            _simulated_speed: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        2006
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        2006
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(2006);
        self._frequency = Default::default();
        self._pulse_length = Default::default();
        self._time_delay = Default::default();
        self._simulated_speed = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_i32_le(self._frequency);
        bfr.put_i32_le(self._pulse_length);
        bfr.put_i32_le(self._time_delay);
        bfr.put_i32_le(self._simulated_speed);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._frequency = bfr.get_i32_le();
        self._pulse_length = bfr.get_i32_le();
        self._time_delay = bfr.get_i32_le();
        self._simulated_speed = bfr.get_i32_le();
        Ok(())
    }
}
