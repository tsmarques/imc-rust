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
use crate::LblBeacon::LblBeacon;
use crate::Message::*;
use crate::DUNE_IMC_CONST_NULL_ID;

/// LBL Beacon position estimate.
#[derive(Default)]
pub struct LblEstimate {
    /// Message Header.
    pub _header: Header,
    /// LBL Beacon Configuration.
    pub _beacon: Option<LblBeacon>,
    /// North position.
    pub _x: f32,
    /// East position.
    pub _y: f32,
    /// North position variance.
    pub _var_x: f32,
    /// East position variance.
    pub _var_y: f32,
    /// Distance.
    pub _distance: f32,
}

impl Message for LblEstimate {
    fn new() -> LblEstimate {
        let msg = LblEstimate {
            _header: Header::new(360),
            _beacon: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _var_x: Default::default(),
            _var_y: Default::default(),
            _distance: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        360
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        360
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(360);
        self._beacon = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._var_x = Default::default();
        self._var_y = Default::default();
        self._distance = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        20
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        inline_message_serialization_size!(dyn_size, self._beacon);

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_inline_message!(bfr, self._beacon);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._var_x);
        bfr.put_f32_le(self._var_y);
        bfr.put_f32_le(self._distance);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._beacon = deserialize_inline_as::<LblBeacon>(bfr).ok();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._var_x = bfr.get_f32_le();
        self._var_y = bfr.get_f32_le();
        self._distance = bfr.get_f32_le();
        Ok(())
    }
}
