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

/// Report of PID control parcels.
#[derive(Default)]
pub struct ControlParcel {
    /// Message Header.
    pub _header: Header,
    /// Proportional Parcel.
    pub _p: f32,
    /// Integrative Parcel.
    pub _i: f32,
    /// Derivative Parcel.
    pub _d: f32,
    /// Anti-Windup Parcel.
    pub _a: f32,
}

impl Message for ControlParcel {
    fn new() -> ControlParcel {
        let msg = ControlParcel {
            _header: Header::new(412),
            _p: Default::default(),
            _i: Default::default(),
            _d: Default::default(),
            _a: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        412
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        412
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(412);
        self._p = Default::default();
        self._i = Default::default();
        self._d = Default::default();
        self._a = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        16
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._i);
        bfr.put_f32_le(self._d);
        bfr.put_f32_le(self._a);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._p = bfr.get_f32_le();
        self._i = bfr.get_f32_le();
        self._d = bfr.get_f32_le();
        self._a = bfr.get_f32_le();
        Ok(())
    }
}
