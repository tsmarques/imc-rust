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

/// This message contains information, collected using USBL, about a
/// target's position.
#[derive(Default, Clone)]
pub struct UsblPositionExtended {
    /// Message Header
    pub _header: Header,
    /// Target's system name.
    pub _target: String,
    /// X coordinate of the target in the local device's reference frame.
    pub _x: f32,
    /// Y coordinate of the target in the local device's reference frame.
    pub _y: f32,
    /// Z coordinate of the target in the local device's reference frame.
    pub _z: f32,
    /// X coordinate of the target in the navigation reference frame.
    pub _n: f32,
    /// Y coordinate of the target in the navigation reference frame.
    pub _e: f32,
    /// Z coordinate of the target in the navigation reference frame.
    pub _d: f32,
    /// Rotation around the device longitudinal axis.
    pub _phi: f32,
    /// Rotation around the device lateral or transverse axis.
    pub _theta: f32,
    /// Rotation around the device vertical axis.
    pub _psi: f32,
    /// Accuracy of the position fix.
    pub _accuracy: f32,
}

impl Message for UsblPositionExtended {
    fn new() -> UsblPositionExtended {
        UsblPositionExtended {
            _header: Header::new(899),
            _target: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _n: Default::default(),
            _e: Default::default(),
            _d: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _accuracy: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        899
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        899
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
        self._header = Header::new(899);
        self._target = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._n = Default::default();
        self._e = Default::default();
        self._d = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._accuracy = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        40
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._target.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._target.as_bytes());
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._n);
        bfr.put_f32_le(self._e);
        bfr.put_f32_le(self._d);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._accuracy);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._target);
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._n = bfr.get_f32_le();
        self._e = bfr.get_f32_le();
        self._d = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._accuracy = bfr.get_f32_le();
        Ok(())
    }
}
