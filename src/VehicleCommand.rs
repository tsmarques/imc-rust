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
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request
    VC_REQUEST = 0,
    /// Reply -- Success
    VC_SUCCESS = 1,
    /// Reply -- In Progress
    VC_IN_PROGRESS = 2,
    /// Reply -- Failure
    VC_FAILURE = 3,
}

/// Command
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Execute Maneuver
    VC_EXEC_MANEUVER = 0,
    /// Stop Maneuver
    VC_STOP_MANEUVER = 1,
    /// Start Calibration
    VC_START_CALIBRATION = 2,
    /// Stop Calibration
    VC_STOP_CALIBRATION = 3,
}

/// Vehicle command.
#[derive(Default, Clone)]
pub struct VehicleCommand {
    /// Message Header
    pub _header: Header,
    pub _type: u8,
    /// Request ID
    pub _request_id: u16,
    /// The type of command/action to be performed
    pub _command: u8,
    /// Maneuver to be executed (for 'EXEC_MANEUVER' command)
    pub _maneuver: Option<Box<dyn Message>>,
    /// Amount of time to calibrate
    pub _calib_time: u16,
    /// Complementary human-readable information for replies.
    pub _info: String,
}

impl Message for VehicleCommand {
    fn new() -> VehicleCommand {
        let msg = VehicleCommand {
            _header: Header::new(501),
            _type: Default::default(),
            _request_id: Default::default(),
            _command: Default::default(),
            _maneuver: Default::default(),
            _calib_time: Default::default(),
            _info: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        501
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        501
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_header()._timestamp = ts;
        if let Some(m) = &mut self._maneuver {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_header()._src = src;
        if let Some(m) = &mut self._maneuver {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_header()._src_ent = src_ent;
        if let Some(m) = &mut self._maneuver {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_header()._dst = dst;
        if let Some(m) = &mut self._maneuver {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_header()._dst_ent = dst_ent;
        if let Some(m) = &mut self._maneuver {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(501);
        self._type = Default::default();
        self._request_id = Default::default();
        self._command = Default::default();
        self._maneuver = Default::default();
        self._calib_time = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._maneuver);
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._request_id);
        bfr.put_u8(self._command);
        serialize_inline_message!(bfr, self._maneuver);
        bfr.put_u16_le(self._calib_time);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._type = bfr.get_u8();
        self._request_id = bfr.get_u16_le();
        self._command = bfr.get_u8();
        self._maneuver = deserialize_inline(bfr).ok();
        self._calib_time = bfr.get_u16_le();
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
