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

/// Acceptance
#[allow(non_camel_case_types)]
pub enum AcceptanceEnum {
    /// Accepted
    RR_ACCEPTED = 0,
    /// Rejected - Above Threshold
    RR_ABOVE_THRESHOLD = 1,
    /// Rejected - Singular Point
    RR_SINGULAR = 2,
    /// Rejected - Not Enough Information
    RR_NO_INFO = 3,
    /// Rejected - Vehicle At Surface
    RR_AT_SURFACE = 4,
}

/// When the vehicle uses Long Base Line navigation, this message
/// notifies that a new range was received from one of the acoustics
/// transponders. The message fields are used to identify the range
/// value and the transponder name. Also, this message has an
/// acceptance field that indicates whether a LBL range was accepted
/// or rejected, and if rejected, the reason why.
#[derive(Default, Clone)]
pub struct LblRangeAcceptance {
    /// Message Header
    pub _header: Header,
    /// Identification number of the acoustic transponder from which
    /// the range information was received.
    pub _id: u8,
    /// Distance to the acoustic transponder.
    pub _range: f32,
    /// Reason for acceptance/rejection.
    pub _acceptance: u8,
}

impl Message for LblRangeAcceptance {
    fn new() -> LblRangeAcceptance {
        let msg = LblRangeAcceptance {
            _header: Header::new(357),
            _id: Default::default(),
            _range: Default::default(),
            _acceptance: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        357
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        357
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(357);
        self._id = Default::default();
        self._range = Default::default();
        self._acceptance = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        6
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._id);
        bfr.put_f32_le(self._range);
        bfr.put_u8(self._acceptance);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._id = bfr.get_u8();
        self._range = bfr.get_f32_le();
        self._acceptance = bfr.get_u8();
        Ok(())
    }
}
