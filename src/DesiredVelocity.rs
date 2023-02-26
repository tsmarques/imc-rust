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
// IMC XML MD5: b521199aa61f91939b6b6ed9e44d149b                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;

use crate::Header::Header;
use crate::Message::*;

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Value of u is meaningful
    pub const FL_SURGE: u32 = 0x01;
    /// Value of v is meaningful
    pub const FL_SWAY: u32 = 0x02;
    /// Value of w is meaningful
    pub const FL_HEAVE: u32 = 0x04;
    /// Value of p is meaningful
    pub const FL_ROLL: u32 = 0x08;
    /// Value of q is meaningful
    pub const FL_PITCH: u32 = 0x10;
    /// Value of r is meaningful
    pub const FL_YAW: u32 = 0x20;
}

/// Desired value for each linear and angular speeds.
#[derive(Default, Clone)]
pub struct DesiredVelocity {
    /// Message Header
    pub _header: Header,
    /// Desired speed along the vehicle's x axis.
    pub _u: f64,
    /// Desired speed along the vehicle's y axis.
    pub _v: f64,
    /// Desired speed along the vehicle's z axis.
    pub _w: f64,
    /// Desired speed about the vehicle's x axis.
    pub _p: f64,
    /// Desired speed about the vehicle's y axis.
    pub _q: f64,
    /// Desired speed about the vehicle's z axis.
    pub _r: f64,
    /// Desired Velocity flags.
    pub _flags: u8,
}

impl Message for DesiredVelocity {
    fn new() -> DesiredVelocity {
        

        DesiredVelocity {
            _header: Header::new(409),
            _u: Default::default(),
            _v: Default::default(),
            _w: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _flags: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        409
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        409
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
        self._header = Header::new(409);
        self._u = Default::default();
        self._v = Default::default();
        self._w = Default::default();
        self._p = Default::default();
        self._q = Default::default();
        self._r = Default::default();
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
        bfr.put_f64_le(self._u);
        bfr.put_f64_le(self._v);
        bfr.put_f64_le(self._w);
        bfr.put_f64_le(self._p);
        bfr.put_f64_le(self._q);
        bfr.put_f64_le(self._r);
        bfr.put_u8(self._flags);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._u = bfr.get_f64_le();
        self._v = bfr.get_f64_le();
        self._w = bfr.get_f64_le();
        self._p = bfr.get_f64_le();
        self._q = bfr.get_f64_le();
        self._r = bfr.get_f64_le();
        self._flags = bfr.get_u8();
        Ok(())
    }
}
