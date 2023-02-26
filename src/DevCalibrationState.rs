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
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Previous Step Not Supported
    pub const DCS_PREVIOUS_NOT_SUPPORTED: u32 = 0x01;
    /// Next Step Not Supported
    pub const DCS_NEXT_NOT_SUPPORTED: u32 = 0x02;
    /// Waiting Device Calibration Control
    pub const DCS_WAITING_CONTROL: u32 = 0x04;
    /// Calibration Error
    pub const DCS_ERROR: u32 = 0x08;
    /// Calibration Procedure Completed
    pub const DCS_COMPLETED: u32 = 0x10;
}

/// State of the calibration procedure.
#[derive(Default, Clone)]
pub struct DevCalibrationState {
    /// Message Header
    pub _header: Header,
    /// Total number of steps of the calibration procedure.
    pub _total_steps: u8,
    /// Number of the current step being performed.
    pub _step_number: u8,
    /// Human-readable description of the current step.
    pub _step: String,
    /// Additional flags.
    pub _flags: u8,
}

impl Message for DevCalibrationState {
    fn new() -> DevCalibrationState {
        

        DevCalibrationState {
            _header: Header::new(13),
            _total_steps: Default::default(),
            _step_number: Default::default(),
            _step: Default::default(),
            _flags: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        13
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        13
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
        self._header = Header::new(13);
        self._total_steps = Default::default();
        self._step_number = Default::default();
        self._step = Default::default();
        self._flags = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._step.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._total_steps);
        bfr.put_u8(self._step_number);
        serialize_bytes!(bfr, self._step.as_bytes());
        bfr.put_u8(self._flags);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._total_steps = bfr.get_u8();
        self._step_number = bfr.get_u8();
        deserialize_string!(bfr, self._step);
        self._flags = bfr.get_u8();
        Ok(())
    }
}
