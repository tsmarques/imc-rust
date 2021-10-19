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
// IMC XML MD5: 9d37efa05563864d61f74279faa9d05f                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Medium
#[allow(non_camel_case_types)]
pub enum MediumEnum {
    /// Ground
    VM_GROUND = 0,
    /// Air
    VM_AIR = 1,
    /// Water
    VM_WATER = 2,
    /// Underwater
    VM_UNDERWATER = 3,
    /// Unknown
    VM_UNKNOWN = 4,
}

/// Detect current vehicle medium.
#[derive(Default)]
pub struct VehicleMedium {
    /// Message Header
    pub _header: Header,
    /// Current medium.
    pub _medium: u8,
}

impl Message for VehicleMedium {
    fn new() -> VehicleMedium {
        let msg = VehicleMedium {
            _header: Header::new(508),
            _medium: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        508
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        508
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(508);
        self._medium = Default::default()
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
        bfr.put_u8(self._medium);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._medium = bfr.get_u8();
        Ok(())
    }
}
