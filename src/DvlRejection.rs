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

/// Reason
#[allow(non_camel_case_types)]
pub enum ReasonEnum {
    /// Innovation Threshold - X
    RR_INNOV_THRESHOLD_X = 0,
    /// Innovation Threshold - Y
    RR_INNOV_THRESHOLD_Y = 1,
    /// Absolute Threshold - X
    RR_ABS_THRESHOLD_X = 2,
    /// Absolute Threshold - Y
    RR_ABS_THRESHOLD_Y = 3,
}

/// Type of velocity
#[allow(non_camel_case_types)]
pub mod TypeofvelocityBits {
    /// Ground velocity
    pub const TYPE_GV: u32 = 0x01;
    /// Water velocity
    pub const TYPE_WV: u32 = 0x02;
}

/// When the vehicle uses Doppler Velocity Log sensor, this message
/// notifies that a new measurement was locally rejected by the
/// navigation filter.
#[derive(Default, Clone)]
pub struct DvlRejection {
    /// Message Header
    pub _header: Header,
    /// This field represents the type of the rejected velocity.
    pub _type: u8,
    /// Reason for rejection. There are two types of DVL measurement
    /// filters. An Innovation filter checks the innovation between
    /// the current measurement and the previous measurement within a
    /// certain amount of time and an Absolute filter compares the
    /// measurement with an absolute threshold value. Those filters
    /// are tested using horizontal speed measurements, i.e.,
    /// measurements in the x-axis and in the y-axis.
    pub _reason: u8,
    /// Value of the rejection.
    /// If it is an innovation rejection the value is
    /// the absolute difference between the previous
    /// accepted DVL measurement and the current one.
    /// If it is an absolute rejection the value is
    /// the current DVL measurement.
    pub _value: f32,
    /// Timestep of the rejection.
    /// The timestep is 0 for an absolute rejection
    /// since it is an instantaneous reading. For
    /// innovation rejection it is the time difference
    /// between the previous accepted DVL measurement
    /// and the current one.
    pub _timestep: f32,
}

impl Message for DvlRejection {
    fn new() -> DvlRejection {
        let msg = DvlRejection {
            _header: Header::new(358),
            _type: Default::default(),
            _reason: Default::default(),
            _value: Default::default(),
            _timestep: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        358
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        358
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(358);
        self._type = Default::default();
        self._reason = Default::default();
        self._value = Default::default();
        self._timestep = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        10
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u8(self._reason);
        bfr.put_f32_le(self._value);
        bfr.put_f32_le(self._timestep);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._reason = bfr.get_u8();
        self._value = bfr.get_f32_le();
        self._timestep = bfr.get_f32_le();
        Ok(())
    }
}
