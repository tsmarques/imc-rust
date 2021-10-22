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
// IMC XML MD5: 3ec4b61a1b487d356bfc62e124f22651                            *
//###########################################################################

/// Base
use bytes::BufMut;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Type
#[allow(non_camel_case_types)]
pub enum TypeEnum {
    /// Stand Alone
    GFT_STANDALONE = 0x00,
    /// Differential
    GFT_DIFFERENTIAL = 0x01,
    /// Dead Reckoning
    GFT_DEAD_RECKONING = 0x02,
    /// Manual Input
    GFT_MANUAL_INPUT = 0x03,
    /// Simulation
    GFT_SIMULATION = 0x04,
}

/// Validity
#[allow(non_camel_case_types)]
pub mod ValidityBits {
    /// Valid Date
    pub const GFV_VALID_DATE: u32 = 0x0001;
    /// Valid Time
    pub const GFV_VALID_TIME: u32 = 0x0002;
    /// Valid Position
    pub const GFV_VALID_POS: u32 = 0x0004;
    /// Valid Course Over Ground
    pub const GFV_VALID_COG: u32 = 0x0008;
    /// Valid Speed Over Ground
    pub const GFV_VALID_SOG: u32 = 0x0010;
    /// Valid Horizontal Accuracy Estimate
    pub const GFV_VALID_HACC: u32 = 0x0020;
    /// Valid Vertical Accuracy Estimate
    pub const GFV_VALID_VACC: u32 = 0x0040;
    /// Valid Horizontal Dilution of Precision
    pub const GFV_VALID_HDOP: u32 = 0x0080;
    /// Valid Vertical Dilution of Precision
    pub const GFV_VALID_VDOP: u32 = 0x0100;
}

/// Report of a GPS fix.
#[derive(Default)]
pub struct GpsFix {
    /// Message Header
    pub _header: Header,
    /// Validity of fields.
    pub _validity: u16,
    /// Type of fix.
    pub _type: u8,
    /// UTC year.
    pub _utc_year: u16,
    /// UTC month.
    pub _utc_month: u8,
    /// UTC day.
    pub _utc_day: u8,
    /// UTC time of the GPS fix measured in seconds since 00:00:00 (midnight).
    pub _utc_time: f32,
    /// WGS-84 Latitude coordinate.
    pub _lat: f64,
    /// WGS-84 Longitude coordinate.
    pub _lon: f64,
    /// Height above WGS-84 ellipsoid.
    pub _height: f32,
    /// Number of satellites used by the GPS device to compute the
    /// solution.
    pub _satellites: u8,
    /// Course Over Ground (true).
    pub _cog: f32,
    /// Speed Over Ground.
    pub _sog: f32,
    /// Horizontal dilution of precision.
    pub _hdop: f32,
    /// Vertical dilution of precision.
    pub _vdop: f32,
    /// Horizontal Accuracy Estimate.
    pub _hacc: f32,
    /// Vertical Accuracy Estimate.
    pub _vacc: f32,
}

impl Message for GpsFix {
    fn new() -> GpsFix {
        let msg = GpsFix {
            _header: Header::new(253),
            _validity: Default::default(),
            _type: Default::default(),
            _utc_year: Default::default(),
            _utc_month: Default::default(),
            _utc_day: Default::default(),
            _utc_time: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _satellites: Default::default(),
            _cog: Default::default(),
            _sog: Default::default(),
            _hdop: Default::default(),
            _vdop: Default::default(),
            _hacc: Default::default(),
            _vacc: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        253
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        253
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(253);
        self._validity = Default::default();
        self._type = Default::default();
        self._utc_year = Default::default();
        self._utc_month = Default::default();
        self._utc_day = Default::default();
        self._utc_time = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._height = Default::default();
        self._satellites = Default::default();
        self._cog = Default::default();
        self._sog = Default::default();
        self._hdop = Default::default();
        self._vdop = Default::default();
        self._hacc = Default::default();
        self._vacc = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        56
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._validity);
        bfr.put_u8(self._type);
        bfr.put_u16_le(self._utc_year);
        bfr.put_u8(self._utc_month);
        bfr.put_u8(self._utc_day);
        bfr.put_f32_le(self._utc_time);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        bfr.put_u8(self._satellites);
        bfr.put_f32_le(self._cog);
        bfr.put_f32_le(self._sog);
        bfr.put_f32_le(self._hdop);
        bfr.put_f32_le(self._vdop);
        bfr.put_f32_le(self._hacc);
        bfr.put_f32_le(self._vacc);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._validity = bfr.get_u16_le();
        self._type = bfr.get_u8();
        self._utc_year = bfr.get_u16_le();
        self._utc_month = bfr.get_u8();
        self._utc_day = bfr.get_u8();
        self._utc_time = bfr.get_f32_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height = bfr.get_f32_le();
        self._satellites = bfr.get_u8();
        self._cog = bfr.get_f32_le();
        self._sog = bfr.get_f32_le();
        self._hdop = bfr.get_f32_le();
        self._vdop = bfr.get_f32_le();
        self._hacc = bfr.get_f32_le();
        self._vacc = bfr.get_f32_le();
        Ok(())
    }
}
