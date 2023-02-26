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

/// Target.
#[derive(Default, Clone)]
pub struct Target {
    /// Message Header
    pub _header: Header,
    /// Target identifier.
    pub _label: String,
    /// WGS-84 Latitude coordinate.
    pub _lat: f64,
    /// WGS-84 Longitude coordinate.
    pub _lon: f64,
    /// Z axis reference. Use z_units to specify whether z represents
    /// depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
    /// Course Over Ground (true).
    pub _cog: f32,
    /// Speed Over Ground.
    pub _sog: f32,
}

impl Message for Target {
    fn new() -> Target {
        

        Target {
            _header: Header::new(800),
            _label: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _cog: Default::default(),
            _sog: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        800
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        800
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
        self._header = Header::new(800);
        self._label = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._cog = Default::default();
        self._sog = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        29
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._label.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._label.as_bytes());
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._cog);
        bfr.put_f32_le(self._sog);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._label);
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._cog = bfr.get_f32_le();
        self._sog = bfr.get_f32_le();
        Ok(())
    }
}
