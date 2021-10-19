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

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Value of X is meaningful
    pub const FL_X: u32 = 0x01;
    /// Value of Y is meaningful
    pub const FL_Y: u32 = 0x02;
    /// Value of Z is meaningful
    pub const FL_Z: u32 = 0x04;
    /// Value of K is meaningful
    pub const FL_K: u32 = 0x08;
    /// Value of M is meaningful
    pub const FL_M: u32 = 0x10;
    /// Value of N is meaningful
    pub const FL_N: u32 = 0x20;
}

/// Set the desired virtual forces and torques to be applied to the
/// vehicle.
#[derive(Default)]
pub struct DesiredControl {
    /// Message Header
    pub _header: Header,
    /// Force X along the vehicle's x axis.
    pub _x: f64,
    /// Force Y along the vehicle's y axis.
    pub _y: f64,
    /// Force Z along the vehicle's z axis.
    pub _z: f64,
    /// Torque K about the vehicle's x axis.
    pub _k: f64,
    /// Torque M about the vehicle's y axis.
    pub _m: f64,
    /// Torque N about the vehicle's z axis.
    pub _n: f64,
    /// Desired Control flags.
    pub _flags: u8,
}

impl Message for DesiredControl {
    fn new() -> DesiredControl {
        let msg = DesiredControl {
            _header: Header::new(407),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _k: Default::default(),
            _m: Default::default(),
            _n: Default::default(),
            _flags: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        407
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        407
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(407);
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._k = Default::default();
        self._m = Default::default();
        self._n = Default::default();
        self._flags = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        49
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
        bfr.put_f64_le(self._k);
        bfr.put_f64_le(self._m);
        bfr.put_f64_le(self._n);
        bfr.put_u8(self._flags);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._x = bfr.get_f64_le();
        self._y = bfr.get_f64_le();
        self._z = bfr.get_f64_le();
        self._k = bfr.get_f64_le();
        self._m = bfr.get_f64_le();
        self._n = bfr.get_f64_le();
        self._flags = bfr.get_u8();
        Ok(())
    }
}
