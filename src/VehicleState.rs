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

/// Operation Mode
#[allow(non_camel_case_types)]
pub enum OperationModeEnum {
    /// Service
    VS_SERVICE = 0,
    /// Calibration
    VS_CALIBRATION = 1,
    /// Error
    VS_ERROR = 2,
    /// Maneuver
    VS_MANEUVER = 3,
    /// External Control
    VS_EXTERNAL = 4,
    /// Boot
    VS_BOOT = 5,
}

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Maneuver Done
    pub const VFLG_MANEUVER_DONE: u32 = 0x01;
}

/// This message summarizes the overall state of the vehicle. It can
/// contains information regarding:
/// - The overall operation mode.
/// - Any error conditions.
/// - Current maneuver execution.
/// - Active control loops.
#[derive(Default, Clone)]
pub struct VehicleState {
    /// Message Header
    pub _header: Header,
    /// The overall operation mode.
    pub _op_mode: u8,
    /// Error count for monitored entitites.
    pub _error_count: u8,
    /// The monitored entities with error conditions. This is a comma
    /// separated list of entity names.
    pub _error_ents: String,
    /// Type of maneuver being executed, when in MANEUVER mode. The
    /// value is the IMC serialization ID of the corresponding
    /// maneuver.
    pub _maneuver_type: u16,
    /// Start time of maneuver being executed (Epoch time), when in
    /// MANEUVER mode.
    pub _maneuver_stime: f64,
    /// Estimated time for maneuver completion. The value will be
    /// 65535 if the time is unknown or undefined.
    pub _maneuver_eta: u16,
    /// Enabled control loops.
    pub _control_loops: u32,
    pub _flags: u8,
    /// Description of last error.
    pub _last_error: String,
    /// Time of last error (Epoch time).
    pub _last_error_time: f64,
}

impl Message for VehicleState {
    fn new() -> VehicleState {
        let msg = VehicleState {
            _header: Header::new(500),
            _op_mode: Default::default(),
            _error_count: Default::default(),
            _error_ents: Default::default(),
            _maneuver_type: Default::default(),
            _maneuver_stime: Default::default(),
            _maneuver_eta: Default::default(),
            _control_loops: Default::default(),
            _flags: Default::default(),
            _last_error: Default::default(),
            _last_error_time: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        500
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        500
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(500);
        self._op_mode = Default::default();
        self._error_count = Default::default();
        self._error_ents = Default::default();
        self._maneuver_type = Default::default();
        self._maneuver_stime = Default::default();
        self._maneuver_eta = Default::default();
        self._control_loops = Default::default();
        self._flags = Default::default();
        self._last_error = Default::default();
        self._last_error_time = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        27
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._error_ents.len() + 2;
        dyn_size += self._last_error.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op_mode);
        bfr.put_u8(self._error_count);
        serialize_bytes!(bfr, self._error_ents.as_bytes());
        bfr.put_u16_le(self._maneuver_type);
        bfr.put_f64_le(self._maneuver_stime);
        bfr.put_u16_le(self._maneuver_eta);
        bfr.put_u32_le(self._control_loops);
        bfr.put_u8(self._flags);
        serialize_bytes!(bfr, self._last_error.as_bytes());
        bfr.put_f64_le(self._last_error_time);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op_mode = bfr.get_u8();
        self._error_count = bfr.get_u8();
        deserialize_string!(bfr, self._error_ents);
        self._maneuver_type = bfr.get_u16_le();
        self._maneuver_stime = bfr.get_f64_le();
        self._maneuver_eta = bfr.get_u16_le();
        self._control_loops = bfr.get_u32_le();
        self._flags = bfr.get_u8();
        deserialize_string!(bfr, self._last_error);
        self._last_error_time = bfr.get_f64_le();
        Ok(())
    }
}
