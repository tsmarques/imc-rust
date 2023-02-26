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
    /// Start Point
    pub const FL_START: u32 = 0x01;
    /// Direct
    pub const FL_DIRECT: u32 = 0x02;
    /// No Altitude/Depth control
    pub const FL_NO_Z: u32 = 0x04;
    /// 3D Tracking
    pub const FL_3DTRACK: u32 = 0x08;
    /// Counter-Clockwise loiter
    pub const FL_CCLOCKW: u32 = 0x10;
    /// Loiter from current position
    pub const FL_LOITER_CURR: u32 = 0x20;
    /// Takeoff
    pub const FL_TAKEOFF: u32 = 0x40;
    /// Land
    pub const FL_LAND: u32 = 0x80;
}

/// Perform path control.
/// The path is specified by two WGS-84 waypoints, respective
/// altitude / depth settings, optional loitering at the end point,
/// and some control flags.
/// The start and end waypoints are defined by the specified end point fields
/// ('end_{lat/lon/z}') plus:
/// 1. the start waypoint fields ('start_{lat|lon|z}') if the
/// 'START' flag (bit in 'flags' field) is set; or
/// 2. the end point of the previous path recently tracked; or
/// 3. the current location is the 'DIRECT' flag is set or if
/// the tracker has been idle for some time.
/// Altitude and depth control can be performed as follows:
/// 1. by default, the tracker will just transmit an altitude/depth
/// reference value equal to 'end_z' to the appropriate controller;
/// 2. if the 'NO_Z' flag is set no altitude/depth control will take
/// place, hence they can be controlled independently;
/// 3. if the '3DTRACK' flag is set, 3D-tracking will be done
/// (if supported by the active controller).
/// Loitering can be specified at the end point with a certain
/// radius ('lradius'), duration ('lduration'), and clockwise or
/// counter-clockwise direction ('CCLOCKW' flag).
#[derive(Default, Clone)]
pub struct DesiredPath {
    /// Message Header
    pub _header: Header,
    /// Unsigned integer reference for the scope of the desired path message.
    /// Path reference should only be set by a maneuver.
    /// Should be set to an always increasing reference at the time of dispatching this message.
    /// Lower level path controllers must inherit the same path reference sent by maneuver.
    pub _path_ref: u32,
    /// WGS-84 latitude of start point. This will be ignored unless
    /// the 'START' flag is set.
    pub _start_lat: f64,
    /// WGS-84 longitude of start point. This will be ignored unless
    /// the 'START' flag is set.
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
    /// Depth or altitude for the end point. This parameter will be
    /// ignored if the 'NO_Z' flag is set.
    pub _end_z: f32,
    /// Units of the end point's z reference.
    pub _end_z_units: u8,
    /// Maneuver speed reference.
    pub _speed: f32,
    /// Speed units.
    pub _speed_units: u8,
    /// Radius for loitering at end point. Specify less or equal to 0
    /// for no loitering.
    pub _lradius: f32,
    /// Desired Path flags.
    pub _flags: u8,
}

impl Message for DesiredPath {
    fn new() -> DesiredPath {
        

        DesiredPath {
            _header: Header::new(406),
            _path_ref: Default::default(),
            _start_lat: Default::default(),
            _start_lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_lat: Default::default(),
            _end_lon: Default::default(),
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _lradius: Default::default(),
            _flags: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        406
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        406
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
        self._header = Header::new(406);
        self._path_ref = Default::default();
        self._start_lat = Default::default();
        self._start_lon = Default::default();
        self._start_z = Default::default();
        self._start_z_units = 0_u8;
        self._end_lat = Default::default();
        self._end_lon = Default::default();
        self._end_z = Default::default();
        self._end_z_units = 0_u8;
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._lradius = Default::default();
        self._flags = Default::default()
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
        bfr.put_u32_le(self._path_ref);
        bfr.put_f64_le(self._start_lat);
        bfr.put_f64_le(self._start_lon);
        bfr.put_f32_le(self._start_z);
        bfr.put_u8(self._start_z_units);
        bfr.put_f64_le(self._end_lat);
        bfr.put_f64_le(self._end_lon);
        bfr.put_f32_le(self._end_z);
        bfr.put_u8(self._end_z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_f32_le(self._lradius);
        bfr.put_u8(self._flags);
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
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._lradius = bfr.get_f32_le();
        self._flags = bfr.get_u8();
        Ok(())
    }
}
