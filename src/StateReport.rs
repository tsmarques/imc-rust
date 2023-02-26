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

/// Concise representation of entire system state.
#[derive(Default, Clone)]
pub struct StateReport {
    /// Message Header
    pub _header: Header,
    /// Time, in seconds, since January 1st 1970.
    pub _stime: u32,
    /// Latitude of the system, in degrees.
    pub _latitude: f32,
    /// Longitude of the system, in degrees.
    pub _longitude: f32,
    /// Altitude of the system, in decimeters.
    /// * *0xFFFF* used for unknown / not applicable value.
    pub _altitude: u16,
    /// Depth of the system, in decimeters.
    /// * *0xFFFF* used for unknown / not applicable value.
    pub _depth: u16,
    /// Calculated as `(rads * (0xFFFF / (2 * PI))`
    pub _heading: u16,
    /// Speed of the system in centimeters per second.
    pub _speed: i16,
    /// System fuel gauge.
    /// * *-1* means unknown fuel level.
    pub _fuel: i8,
    /// Progress of execution or idle state.
    /// * *-1* means Service mode
    /// * *-2* means Boot mode
    /// * *-3* means Calibration mode
    /// * *-4* means Error mode
    pub _exec_state: i8,
    /// Checksum of the plan being executed.
    pub _plan_checksum: u16,
}

impl Message for StateReport {
    fn new() -> StateReport {
        

        StateReport {
            _header: Header::new(514),
            _stime: Default::default(),
            _latitude: Default::default(),
            _longitude: Default::default(),
            _altitude: Default::default(),
            _depth: Default::default(),
            _heading: Default::default(),
            _speed: Default::default(),
            _fuel: Default::default(),
            _exec_state: Default::default(),
            _plan_checksum: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        514
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        514
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
        self._header = Header::new(514);
        self._stime = Default::default();
        self._latitude = Default::default();
        self._longitude = Default::default();
        self._altitude = Default::default();
        self._depth = Default::default();
        self._heading = Default::default();
        self._speed = Default::default();
        self._fuel = Default::default();
        self._exec_state = Default::default();
        self._plan_checksum = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        24
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._stime);
        bfr.put_f32_le(self._latitude);
        bfr.put_f32_le(self._longitude);
        bfr.put_u16_le(self._altitude);
        bfr.put_u16_le(self._depth);
        bfr.put_u16_le(self._heading);
        bfr.put_i16_le(self._speed);
        bfr.put_i8(self._fuel);
        bfr.put_i8(self._exec_state);
        bfr.put_u16_le(self._plan_checksum);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._stime = bfr.get_u32_le();
        self._latitude = bfr.get_f32_le();
        self._longitude = bfr.get_f32_le();
        self._altitude = bfr.get_u16_le();
        self._depth = bfr.get_u16_le();
        self._heading = bfr.get_u16_le();
        self._speed = bfr.get_i16_le();
        self._fuel = bfr.get_i8();
        self._exec_state = bfr.get_i8();
        self._plan_checksum = bfr.get_u16_le();
        Ok(())
    }
}
