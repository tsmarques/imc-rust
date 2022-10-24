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

/// Enable
#[allow(non_camel_case_types)]
pub enum EnableEnum {
    /// Disable
    CL_DISABLE = 0,
    /// Enable
    CL_ENABLE = 1,
}

/// Enable or disable control loops.
#[derive(Default, Clone)]
pub struct ControlLoops {
    /// Message Header
    pub _header: Header,
    pub _enable: u8,
    /// Control loop mask.
    pub _mask: u32,
    /// Unsigned integer reference for the scope of the control loop message.
    /// Scope reference should only be set by a maneuver.
    /// Should be set to an always increasing reference at the time of dispatching this message.
    /// Lower level controllers must inherit the same scope reference sent by maneuver.
    /// This same scope reference must be sent down to lower control layers.
    pub _scope_ref: u32,
}

impl Message for ControlLoops {
    fn new() -> ControlLoops {
        let msg = ControlLoops {
            _header: Header::new(507),
            _enable: Default::default(),
            _mask: Default::default(),
            _scope_ref: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        507
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        507
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(507);
        self._enable = Default::default();
        self._mask = Default::default();
        self._scope_ref = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._enable);
        bfr.put_u32_le(self._mask);
        bfr.put_u32_le(self._scope_ref);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._enable = bfr.get_u8();
        self._mask = bfr.get_u32_le();
        self._scope_ref = bfr.get_u32_le();
        Ok(())
    }
}
