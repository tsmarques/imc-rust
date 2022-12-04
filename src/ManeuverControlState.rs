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

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// State
#[allow(non_camel_case_types)]
pub enum StateEnum {
    /// Maneuver in progress
    MCS_EXECUTING = 0,
    /// Maneuver completed
    MCS_DONE = 1,
    /// Maneuver error
    MCS_ERROR = 2,
    /// Maneuver stopped
    MCS_STOPPED = 3,
}

/// Maneuver control state.
#[derive(Default, Clone)]
pub struct ManeuverControlState {
    /// Message Header
    pub _header: Header,
    /// Code indicating maneuver state.
    pub _state: u8,
    /// Estimated time to completion of the maneuver, when executing.
    /// The value will be 65535 if the time is unknown or undefined.
    pub _eta: u16,
    /// Complementary information, e.g., regarding errors.
    pub _info: String,
}

impl Message for ManeuverControlState {
    fn new() -> ManeuverControlState {
        

        ManeuverControlState {
            _header: Header::new(470),
            _state: Default::default(),
            _eta: Default::default(),
            _info: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        470
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        470
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
        self._header = Header::new(470);
        self._state = Default::default();
        self._eta = Default::default();
        self._info = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        3
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._info.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._state);
        bfr.put_u16_le(self._eta);
        serialize_bytes!(bfr, self._info.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        self._eta = bfr.get_u16_le();
        deserialize_string!(bfr, self._info);
        Ok(())
    }
}
