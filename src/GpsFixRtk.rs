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

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// None
    RTK_NONE = 0x00,
    /// Obs
    RTK_OBS = 0x01,
    /// Float
    RTK_FLOAT = 0x02,
    /// Fixed
    RTK_FIXED = 0x03,
}

/// Validity
#[allow(non_camel_case_types)]
pub mod ValidityBits {
    /// Valid Time
    pub const RFV_VALID_TIME: u32 = 0x0001;
    /// Valid Base LLH
    pub const RFV_VALID_BASE: u32 = 0x0002;
    /// Valid Position
    pub const RFV_VALID_POS: u32 = 0x0004;
    /// Valid Velocity
    pub const RFV_VALID_VEL: u32 = 0x0008;
}

/// Report of an RTK-GPS fix.
#[derive(Default, Clone)]
pub struct GpsFixRtk {
    /// Message Header
    pub _header: Header,
    /// Validity of fields.
    pub _validity: u16,
    /// Type of fix.
    pub _type: u8,
    /// GPS Time of Week.
    pub _tow: u32,
    /// WGS-84 Latitude coordinate of the base.
    pub _base_lat: f64,
    /// WGS-84 Longitude coordinate of the base.
    pub _base_lon: f64,
    /// Height above WGS-84 ellipsoid of the base.
    pub _base_height: f32,
    /// Baseline North coordinate.
    pub _n: f32,
    /// Baseline East coordinate.
    pub _e: f32,
    /// Baseline Down coordinate.
    pub _d: f32,
    /// Velocity North coordinate.
    pub _v_n: f32,
    /// Velocity East coordinate.
    pub _v_e: f32,
    /// Velocity Down coordinate.
    pub _v_d: f32,
    /// Number of satellites used in solution.
    pub _satellites: u8,
    /// Number of hypotheses in the Integer Ambiguity Resolution (smaller is better).
    pub _iar_hyp: u16,
    /// Quality ratio of Integer Ambiguity Resolution (bigger is better).
    pub _iar_ratio: f32,
}

impl Message for GpsFixRtk {
    fn new() -> GpsFixRtk {
        

        GpsFixRtk {
            _header: Header::new(293),
            _validity: Default::default(),
            _type: Default::default(),
            _tow: Default::default(),
            _base_lat: Default::default(),
            _base_lon: Default::default(),
            _base_height: Default::default(),
            _n: Default::default(),
            _e: Default::default(),
            _d: Default::default(),
            _v_n: Default::default(),
            _v_e: Default::default(),
            _v_d: Default::default(),
            _satellites: Default::default(),
            _iar_hyp: Default::default(),
            _iar_ratio: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        293
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        293
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
        self._header = Header::new(293);
        self._validity = Default::default();
        self._type = Default::default();
        self._tow = Default::default();
        self._base_lat = Default::default();
        self._base_lon = Default::default();
        self._base_height = Default::default();
        self._n = Default::default();
        self._e = Default::default();
        self._d = Default::default();
        self._v_n = Default::default();
        self._v_e = Default::default();
        self._v_d = Default::default();
        self._satellites = Default::default();
        self._iar_hyp = Default::default();
        self._iar_ratio = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        58
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._validity);
        bfr.put_u8(self._type);
        bfr.put_u32_le(self._tow);
        bfr.put_f64_le(self._base_lat);
        bfr.put_f64_le(self._base_lon);
        bfr.put_f32_le(self._base_height);
        bfr.put_f32_le(self._n);
        bfr.put_f32_le(self._e);
        bfr.put_f32_le(self._d);
        bfr.put_f32_le(self._v_n);
        bfr.put_f32_le(self._v_e);
        bfr.put_f32_le(self._v_d);
        bfr.put_u8(self._satellites);
        bfr.put_u16_le(self._iar_hyp);
        bfr.put_f32_le(self._iar_ratio);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u16_le();
        self._type = bfr.get_u8();
        self._tow = bfr.get_u32_le();
        self._base_lat = bfr.get_f64_le();
        self._base_lon = bfr.get_f64_le();
        self._base_height = bfr.get_f32_le();
        self._n = bfr.get_f32_le();
        self._e = bfr.get_f32_le();
        self._d = bfr.get_f32_le();
        self._v_n = bfr.get_f32_le();
        self._v_e = bfr.get_f32_le();
        self._v_d = bfr.get_f32_le();
        self._satellites = bfr.get_u8();
        self._iar_hyp = bfr.get_u16_le();
        self._iar_ratio = bfr.get_f32_le();
        Ok(())
    }
}
