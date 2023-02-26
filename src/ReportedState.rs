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

/// Source Type
#[allow(non_camel_case_types)]
pub enum SourceTypeEnum {
    /// Wi-Fi
    STYPE_WI_FI = 0,
    /// Tracker
    STYPE_TRACKER = 1,
    /// SMS
    STYPE_SMS = 2,
    /// Acoustic Modem
    STYPE_ACOUSTIC_MODEM = 3,
    /// Unknown source
    STYPE_UNKNOWN = 254,
}

/// A vehicle state that is reported to other consoles (including PDAConsole). Source can be acoustic tracker, SMS, Wi-Fi, etc...
#[derive(Default, Clone)]
pub struct ReportedState {
    /// Message Header
    pub _header: Header,
    pub _lat: f64,
    pub _lon: f64,
    /// The reported depth. In the case of not knowing the depth 0 will be reported.
    /// Airplanes usually have negative values (por positive altitude).
    pub _depth: f64,
    /// The phi Euler angle from the vehicle's attitude.
    pub _roll: f64,
    /// The theta Euler angle from the vehicle's attitude.
    pub _pitch: f64,
    /// The psi Euler angle from the vehicle's attitude.
    pub _yaw: f64,
    /// The time when the packet was sent, as seen by the packet
    /// dispatcher. The number of seconds is represented in Universal
    /// Coordinated Time (UCT) in seconds since Jan 1, 1970 using IEEE
    /// double precision floating point numbers.
    pub _rcp_time: f64,
    /// The id of the system whose position is being reported (it can be a vehicle's id, a boat name, etc)
    pub _sid: String,
    /// How the position was received/calculated
    pub _s_type: u8,
}

impl Message for ReportedState {
    fn new() -> ReportedState {
        

        ReportedState {
            _header: Header::new(600),
            _lat: Default::default(),
            _lon: Default::default(),
            _depth: Default::default(),
            _roll: Default::default(),
            _pitch: Default::default(),
            _yaw: Default::default(),
            _rcp_time: Default::default(),
            _sid: Default::default(),
            _s_type: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        600
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        600
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
        self._header = Header::new(600);
        self._lat = Default::default();
        self._lon = Default::default();
        self._depth = Default::default();
        self._roll = Default::default();
        self._pitch = Default::default();
        self._yaw = Default::default();
        self._rcp_time = Default::default();
        self._sid = Default::default();
        self._s_type = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        57
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._sid.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f64_le(self._depth);
        bfr.put_f64_le(self._roll);
        bfr.put_f64_le(self._pitch);
        bfr.put_f64_le(self._yaw);
        bfr.put_f64_le(self._rcp_time);
        serialize_bytes!(bfr, self._sid.as_bytes());
        bfr.put_u8(self._s_type);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._depth = bfr.get_f64_le();
        self._roll = bfr.get_f64_le();
        self._pitch = bfr.get_f64_le();
        self._yaw = bfr.get_f64_le();
        self._rcp_time = bfr.get_f64_le();
        deserialize_string!(bfr, self._sid);
        self._s_type = bfr.get_u8();
        Ok(())
    }
}
