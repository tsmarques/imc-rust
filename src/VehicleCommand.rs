//###########################################################################
// Copyright 2017 OceanScan - Marine Systems & Technology, Lda.             #
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
// Author: Ricardo Martins                                                  #
//###########################################################################
// Automatically generated.                                                 *
//###########################################################################
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Author: Tiago Sá Marques <tmarques@oceanscan-mst.com>

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Type.
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Request.
    VC_REQUEST = 0,
    /// Reply -- Success.
    VC_SUCCESS = 1,
    /// Reply -- In Progress.
    VC_IN_PROGRESS = 2,
    /// Reply -- Failure.
    VC_FAILURE = 3,
}

/// Command.
#[allow(non_camel_case_types)]
pub enum CommandEnum {
    /// Execute Maneuver.
    VC_EXEC_MANEUVER = 0,
    /// Stop Maneuver.
    VC_STOP_MANEUVER = 1,
    /// Start Calibration.
    VC_START_CALIBRATION = 2,
    /// Stop Calibration.
    VC_STOP_CALIBRATION = 3,
}

/// Vehicle command.
#[derive(Default)]
pub struct VehicleCommand {
    /// Message Header.
    pub _header: Header,
    /// Type.
    pub _type: u8,
    /// Request ID.
    pub _request_id: u16,
    /// Command.
    pub _command: u8,
    /// Maneuver.
    pub _maneuver: Option<Box<dyn Message>>,
    /// Calibration Time.
    pub _calib_time: u16,
    /// Info.
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
