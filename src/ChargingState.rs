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

/// Is Charging
#[allow(non_camel_case_types)]
pub enum IsChargingEnum {
    /// Not Charging
    CSTATE_NOT_CHARGING = 0,
    /// Is Charging
    CSTATE_IS_CHARGING = 1,
}

/// Reports if the vehicle is charging or not
#[derive(Default, Clone)]
pub struct ChargingState {
    /// Message Header
    pub _header: Header,
    pub _is_charging: u8,
}

impl Message for ChargingState {
    fn new() -> ChargingState {
        

        ChargingState {
            _header: Header::new(315),
            _is_charging: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        315
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        315
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
        self._header = Header::new(315);
        self._is_charging = Default::default()
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
        bfr.put_u8(self._is_charging);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._is_charging = bfr.get_u8();
        Ok(())
    }
}
