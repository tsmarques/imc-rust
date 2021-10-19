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

/// Reason
#[allow(non_camel_case_types)]
pub enum ReasonEnum {
    /// Above Threshold
    RR_ABOVE_THRESHOLD = 0,
    /// Invalid Fix
    RR_INVALID = 1,
    /// Above Maximum HDOP
    RR_ABOVE_MAX_HDOP = 2,
    /// Above Maximum HACC
    RR_ABOVE_MAX_HACC = 3,
    /// Lost Validity Bit
    RR_LOST_VAL_BIT = 4,
}

#[derive(Default)]
pub struct GpsFixRejection {
    /// Message Header
    pub _header: Header,
    /// UTC time of the rejected GPS fix measured in seconds since
    /// 00:00:00 (midnight).
    pub _utc_time: f32,
    /// Reason for rejection.
    pub _reason: u8,
}

impl Message for GpsFixRejection {
    fn new() -> GpsFixRejection {
        let msg = GpsFixRejection {
            _header: Header::new(356),
            _utc_time: Default::default(),
            _reason: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        356
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        356
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(356);
        self._utc_time = Default::default();
        self._reason = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        5
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._utc_time);
        bfr.put_u8(self._reason);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._utc_time = bfr.get_f32_le();
        self._reason = bfr.get_u8();
        Ok(())
    }
}
