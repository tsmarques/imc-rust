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

use crate::Header;
use crate::Message;

/// Position Mismatch Monitor
#[allow(non_camel_case_types)]
pub enum PositionMismatchMonitorEnum {
    /// Ok
    POS_OK = 0,
    /// Warning threshold
    POS_WRN = 1,
    /// Limit threshold
    POS_LIM = 2,
}

/// Communications Monitor
#[allow(non_camel_case_types)]
pub enum CommunicationsMonitorEnum {
    /// Ok
    COMMS_OK = 0,
    /// Timeout
    COMMS_TIMEOUT = 1,
}

/// Convergence
#[allow(non_camel_case_types)]
pub enum ConvergenceEnum {
    /// Ok
    CONV_OK = 0,
    /// Timeout
    CONV_TIMEOUT = 1,
}

/// Monitoring variables to assert the formation tracking state, i.e., the mismatch between the real and the simulated aircraft position, the convergence state, etc.
#[derive(Default, Clone)]
pub struct FormState {
    /// Message Header
    pub _header: Header,
    /// Mismatch between the real and the simulated aircraft position.
    pub _PosSimErr: f32,
    /// Convergence evalution variable.
    /// Value indicates the position error to which the system is converging, tacking into account the aircraft current position error and velocity.
    pub _Converg: f32,
    /// Evaluation of the stream turbulence level, through the stream acceleration.
    pub _Turbulence: f32,
    /// Position mismatch monitoring flag.
    pub _PosSimMon: u8,
    /// Communications monitoring flag.
    pub _CommMon: u8,
    /// Convergence monitoring flag.
    pub _ConvergMon: u8,
}

impl Message for FormState {
    fn new() -> FormState {
        FormState {
            _header: Header::new(510),
            _PosSimErr: Default::default(),
            _Converg: Default::default(),
            _Turbulence: Default::default(),
            _PosSimMon: Default::default(),
            _CommMon: Default::default(),
            _ConvergMon: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        510
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        510
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(510);
        self._PosSimErr = Default::default();
        self._Converg = Default::default();
        self._Turbulence = Default::default();
        self._PosSimMon = Default::default();
        self._CommMon = Default::default();
        self._ConvergMon = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        15
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._PosSimErr);
        bfr.put_f32_le(self._Converg);
        bfr.put_f32_le(self._Turbulence);
        bfr.put_u8(self._PosSimMon);
        bfr.put_u8(self._CommMon);
        bfr.put_u8(self._ConvergMon);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._PosSimErr = bfr.get_f32_le();
        self._Converg = bfr.get_f32_le();
        self._Turbulence = bfr.get_f32_le();
        self._PosSimMon = bfr.get_u8();
        self._CommMon = bfr.get_u8();
        self._ConvergMon = bfr.get_u8();
        Ok(())
    }
}
