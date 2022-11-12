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
// IMC XML MD5: 732df4108a86978f313ac1bb5a1f55eb                            *
//###########################################################################

use bytes::BufMut;
/// Base
use std::any::Any;

use crate::packet::ImcError;
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Start from current position
    pub const FLG_CURR_POS: u32 = 0x01;
}

/// The Elevator maneuver specifies a vehicle to reach a target
/// waypoint using a cruise altitude/depth and then ascend or
/// descend to another target altitude/depth. The ascent/descent
/// slope and radius can also be optionally specified.
#[derive(Default, Clone)]
pub struct Elevator {
    /// Message Header
    pub _header: Header,
    /// The amount of time the maneuver is allowed to run. If the
    /// maneuver is not completed in the amount of time specified an
    /// error will be generated.
    pub _timeout: u16,
    /// Flags of the maneuver.
    pub _flags: u8,
    /// WGS-84 Latitude.
    pub _lat: f64,
    /// WGS-84 Longitude.
    pub _lon: f64,
    /// Altitude or depth of start point. This parameter will be
    /// ignored if the 'NO_Z' flag is set, or if the 'START' flag is
    /// not set.
    pub _start_z: f32,
    /// Units of the start point's z reference.
    pub _start_z_units: u8,
    /// Depth or altitude for the end point.  This parameter will be
    /// ignored if the 'NO_Z' flag is set.
    pub _end_z: f32,
    /// Units of the end point's z reference.
    pub _end_z_units: u8,
    /// Radius for use in ascent/descent. If 0 the controller to
    /// should use a custom control strategy.
    pub _radius: f32,
    /// Maneuver speed.
    pub _speed: f32,
    /// Speed units.
    pub _speed_units: u8,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for Elevator {
    fn new() -> Elevator {
        let msg = Elevator {
            _header: Header::new(462),
            _timeout: Default::default(),
            _flags: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _start_z: Default::default(),
            _start_z_units: 0_u8,
            _end_z: Default::default(),
            _end_z_units: 0_u8,
            _radius: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _custom: Default::default(),
        };

        msg
    }

    #[inline(always)]
    fn static_id() -> u16 {
        462
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        462
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
        self._header = Header::new(462);
        self._timeout = Default::default();
        self._flags = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._start_z = Default::default();
        self._start_z_units = 0_u8;
        self._end_z = Default::default();
        self._end_z_units = 0_u8;
        self._radius = Default::default();
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        38
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._timeout);
        bfr.put_u8(self._flags);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._start_z);
        bfr.put_u8(self._start_z_units);
        bfr.put_f32_le(self._end_z);
        bfr.put_u8(self._end_z_units);
        bfr.put_f32_le(self._radius);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._timeout = bfr.get_u16_le();
        self._flags = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._start_z = bfr.get_f32_le();
        self._start_z_units = bfr.get_u8();
        self._end_z = bfr.get_f32_le();
        self._end_z_units = bfr.get_u8();
        self._radius = bfr.get_f32_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
