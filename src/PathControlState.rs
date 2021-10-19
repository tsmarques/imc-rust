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
use crate::Message::*;

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Near Endpoint
    pub const FL_NEAR: u32 = 0x01;
    /// Loitering
    pub const FL_LOITERING: u32 = 0x02;
    /// No Altitude/Depth control
    pub const FL_NO_Z: u32 = 0x04;
    /// 3D Tracking
    pub const FL_3DTRACK: u32 = 0x08;
    /// Counter-Clockwise loiter
    pub const FL_CCLOCKW: u32 = 0x10;
}

/// Path control state issued by Path Controller.
#[derive(Default)]
pub struct PathControlState {
    /// Message Header
    pub _header: Header,
    /// Unsigned integer reference of the desired path message to which this
    /// PathControlState message refers to.
    /// Path reference should only be set by a maneuver, not by path controllers.
    pub _path_ref: u32,
    /// WGS-84 latitude of start point.
    pub _start_lat: f64,
    /// WGS-84 longitude of start point.
    pub _start_lon: f64,
    /// Altitude or depth of start point. This parameter will be
    /// ignored if the 'NO_Z' flag is set, or if the 'START' flag is
    /// not set.
    pub _start_z: f32,
    /// Units of the start point's z reference.
    pub _start_z_units: u8,
    /// WGS-84 latitude of end point.
    pub _end_lat: f64,
    /// WGS-84 longitude of end point.
    pub _end_lon: f64,
    /// Depth or altitude for the end point. This parameter should be
    /// ignored if the 'NO_Z' flag is set.
    pub _end_z: f32,
    /// Units of the end point's z reference.
    pub _end_z_units: u8,
    /// Radius for loitering at end point.
    /// Will be 0 if no loitering is active.
    pub _lradius: f32,
    /// Path control state flags.
    pub _flags: u8,
    /// Along-Track position value.
    pub _x: f32,
    /// Cross-Track position value.
    pub _y: f32,
    /// Vertical-Track position value.
    pub _z: f32,
    /// Along-Track velocity value.
    pub _vx: f32,
    /// Cross-Track velocity value.
    pub _vy: f32,
    /// Vertical-Track velocity value.
    pub _vz: f32,
    /// Course error value.
    pub _course_error: f32,
    /// Estimated time to reach target waypoint. The value will be
    /// 65535 if the time is unknown or undefined, and 0 when
    /// loitering.
    pub _eta: u16,
}

impl Message for PathControlState {
    fn new() -> PathControlState {
        let msg = PathControlState {
            _header: Header::new(410),
            _path_ref: Default::default(),
            _start_lat: Default::default(),
            _start_lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_lat: Default::default(),
            _end_lon: Default::default(),
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _lradius: Default::default(),
            _flags: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _vx: Default::default(),
            _vy: Default::default(),
            _vz: Default::default(),
            _course_error: Default::default(),
            _eta: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        410
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        410
    }

    fn get_header(&mut self) -> &mut Header {
        &mut self._header
    }

    fn clear(&mut self) {
        self._header = Header::new(410);
        self._path_ref = Default::default();
        self._start_lat = Default::default();
        self._start_lon = Default::default();
        self._start_z = Default::default();
        self._start_z_units = 0_u8;
        self._end_lat = Default::default();
        self._end_lon = Default::default();
        self._end_z = Default::default();
        self._end_z_units = 0_u8;
        self._lradius = Default::default();
        self._flags = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._vx = Default::default();
        self._vy = Default::default();
        self._vz = Default::default();
        self._course_error = Default::default();
        self._eta = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        81
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u32_le(self._path_ref);
        bfr.put_f64_le(self._start_lat);
        bfr.put_f64_le(self._start_lon);
        bfr.put_f32_le(self._start_z);
        bfr.put_u8(self._start_z_units);
        bfr.put_f64_le(self._end_lat);
        bfr.put_f64_le(self._end_lon);
        bfr.put_f32_le(self._end_z);
        bfr.put_u8(self._end_z_units);
        bfr.put_f32_le(self._lradius);
        bfr.put_u8(self._flags);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._vx);
        bfr.put_f32_le(self._vy);
        bfr.put_f32_le(self._vz);
        bfr.put_f32_le(self._course_error);
        bfr.put_u16_le(self._eta);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._path_ref = bfr.get_u32_le();
        self._start_lat = bfr.get_f64_le();
        self._start_lon = bfr.get_f64_le();
        self._start_z = bfr.get_f32_le();
        self._start_z_units = bfr.get_u8();
        self._end_lat = bfr.get_f64_le();
        self._end_lon = bfr.get_f64_le();
        self._end_z = bfr.get_f32_le();
        self._end_z_units = bfr.get_u8();
        self._lradius = bfr.get_f32_le();
        self._flags = bfr.get_u8();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._vx = bfr.get_f32_le();
        self._vy = bfr.get_f32_le();
        self._vz = bfr.get_f32_le();
        self._course_error = bfr.get_f32_le();
        self._eta = bfr.get_u16_le();
        Ok(())
    }
}
