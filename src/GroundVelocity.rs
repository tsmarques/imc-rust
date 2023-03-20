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

/// Validity
#[allow(non_camel_case_types)]
pub mod ValidityBits {
    /// X component is valid
    pub const VAL_VEL_X: u32 = 0x01;
    /// Y component is valid
    pub const VAL_VEL_Y: u32 = 0x02;
    /// Z component is valid
    pub const VAL_VEL_Z: u32 = 0x04;
}

/// Vector quantifying the direction and magnitude of the measured
/// velocity relative to the ground that a device is exposed to.
#[derive(Default, Clone)]
pub struct GroundVelocity {
    /// Message Header
    pub _header: Header,
    /// Each bit of this field represents if a given velocity
    /// component is valid.
    pub _validity: u8,
    /// X component.
    pub _x: f64,
    /// Y component.
    pub _y: f64,
    /// Z component.
    pub _z: f64,
}

impl Message for GroundVelocity {
    fn new() -> GroundVelocity {
        GroundVelocity {
            _header: Header::new(259),
            _validity: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        259
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        259
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_header(&self) -> &Header {
        &self._header
    }

    fn get_mut_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(259);
        self._validity = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        25
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._validity);
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u8();
        self._x = bfr.get_f64_le();
        self._y = bfr.get_f64_le();
        self._z = bfr.get_f64_le();
        Ok(())
    }
}
