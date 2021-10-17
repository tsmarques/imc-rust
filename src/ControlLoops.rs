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

/// Enable.
#[allow(non_camel_case_types)]
pub enum EnableEnum {
    /// Disable.
    CL_DISABLE = 0,
    /// Enable.
    CL_ENABLE = 1,
}

/// Enable or disable control loops.
#[derive(Default)]
pub struct ControlLoops {
    /// Message Header.
    pub _header: Header,
    /// Enable.
    pub _enable: u8,
    /// Control Loop Mask.
    pub _mask: u32,
    /// Scope Time Reference.
    pub _scope_ref: u32,
}

impl Message for ControlLoops {
    fn new() -> ControlLoops
    where
        Self: Sized,
    {
        let msg = ControlLoops {
            _header: Header::new(507),
            _enable: Default::default(),
            _mask: Default::default(),
            _scope_ref: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16
    where
        Self: Sized,
    {
        507
    }

    #[inline(always)]
    fn id(&self) -> u16
    where
        Self: Sized,
    {
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
