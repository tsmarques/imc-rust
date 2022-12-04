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
    /// Not Aligned
    AS_NOT_ALIGNED = 0,
    /// Aligned
    AS_ALIGNED = 1,
    /// Not Supported
    AS_NOT_SUPPORTED = 2,
    /// Aligning
    AS_ALIGNING = 3,
    /// Wrong Medium
    AS_WRONG_MEDIUM = 4,
    /// Coarse Alignment
    AS_COARSE_ALIGNMENT = 5,
    /// Fine Alignment
    AS_FINE_ALIGNMENT = 6,
    /// System Ready
    AS_SYSTEM_READY = 7,
}

/// This message notifies the vehicle is ready for dead-reckoning missions.
#[derive(Default, Clone)]
pub struct AlignmentState {
    /// Message Header
    pub _header: Header,
    /// Alignment State.
    pub _state: u8,
}

impl Message for AlignmentState {
    fn new() -> AlignmentState {
        

        AlignmentState {
            _header: Header::new(361),
            _state: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        361
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        361
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
        self._header = Header::new(361);
        self._state = Default::default()
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
        bfr.put_u8(self._state);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._state = bfr.get_u8();
        Ok(())
    }
}
