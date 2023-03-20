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

use crate::Header;
use crate::Message;

/// Action on the vehicle home position
#[allow(non_camel_case_types)]
pub enum ActiononthevehiclehomepositionEnum {
    /// Set
    OP_SET = 1,
    /// Report
    OP_REPORT = 2,
}

/// Vehicle Home Position.
#[derive(Default, Clone)]
pub struct HomePosition {
    /// Message Header
    pub _header: Header,
    /// Action on the vehicle home
    pub _op: u8,
    /// WGS-84 Latitude.
    pub _lat: f64,
    /// WGS-84 Longitude.
    pub _lon: f64,
    /// Height above the WGS-84 ellipsoid.
    pub _height: f32,
    /// Depth, in meters. To be used by underwater vehicles. Negative
    /// values denote invalid estimates.
    pub _depth: f32,
    /// Altitude, in meters. Negative values denote invalid estimates.
    pub _alt: f32,
}

impl Message for HomePosition {
    fn new() -> HomePosition {
        HomePosition {
            _header: Header::new(909),
            _op: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _depth: Default::default(),
            _alt: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        909
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        909
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
        self._header = Header::new(909);
        self._op = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._height = Default::default();
        self._depth = Default::default();
        self._alt = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        29
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        bfr.put_f32_le(self._depth);
        bfr.put_f32_le(self._alt);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height = bfr.get_f32_le();
        self._depth = bfr.get_f32_le();
        self._alt = bfr.get_f32_le();
        Ok(())
    }
}
