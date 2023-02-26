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

/// Operation
#[allow(non_camel_case_types)]
pub enum OperationEnum {
    /// Start
    DCAL_START = 0,
    /// Stop
    DCAL_STOP = 1,
    /// Perform Next Calibration Step
    DCAL_STEP_NEXT = 2,
    /// Perform Previous Calibration Step
    DCAL_STEP_PREVIOUS = 3,
}

/// This message controls the calibration procedure of a given
/// device. The destination device is selected using the destination
/// entity identification number.
#[derive(Default, Clone)]
pub struct DevCalibrationControl {
    /// Message Header
    pub _header: Header,
    /// Operation to perform.
    pub _op: u8,
}

impl Message for DevCalibrationControl {
    fn new() -> DevCalibrationControl {
        

        DevCalibrationControl {
            _header: Header::new(12),
            _op: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        12
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        12
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
        self._header = Header::new(12);
        self._op = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        1
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        Ok(())
    }
}
