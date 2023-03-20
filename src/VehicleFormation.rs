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
use crate::packet::*;
use crate::Header::Header;
use crate::Message::*;
use crate::MessageList;
use crate::TrajectoryPoint::TrajectoryPoint;
use crate::VehicleFormationParticipant::VehicleFormationParticipant;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Coordinate maneuver using two or more cooperating systems.
#[derive(Default, Clone)]
pub struct VehicleFormation {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude for start point.
    pub _lat: f64,
    /// WGS-84 Longitude for start point.
    pub _lon: f64,
    /// Maneuver reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the z reference.
    pub _z_units: u8,
    /// Reference speed.
    pub _speed: f32,
    /// Reference speed units.
    pub _speed_units: u8,
    /// List of trajectory points.
    pub _points: MessageList<TrajectoryPoint>,
    /// List of formation participants.
    pub _participants: MessageList<VehicleFormationParticipant>,
    /// Optional start time hint for vehicle formation.
    pub _start_time: f64,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for VehicleFormation {
    fn new() -> VehicleFormation {
        VehicleFormation {
            _header: Header::new(466),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _speed: Default::default(),
            _speed_units: 0_u8,
            _points: Default::default(),
            _participants: Default::default(),
            _start_time: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        466
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        466
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

    fn set_timestamp_secs(&mut self, ts: f64) {
        self.get_mut_header()._timestamp = ts;
        for m in &mut self._points {
            m.set_timestamp_secs(ts);
        }
        for m in &mut self._participants {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._points {
            m.set_source(src);
        }
        for m in &mut self._participants {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._points {
            m.set_source_ent(src_ent);
        }
        for m in &mut self._participants {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._points {
            m.set_destination(dst);
        }
        for m in &mut self._participants {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._points {
            m.set_destination_ent(dst_ent);
        }
        for m in &mut self._participants {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(466);
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._points = Default::default();
        self._participants = Default::default();
        self._start_time = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        34
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._points);
        message_list_serialization_size!(dyn_size, self._participants);
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        serialize_message_list!(bfr, self._points);
        serialize_message_list!(bfr, self._participants);
        bfr.put_f64_le(self._start_time);
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._points = deserialize_message_list_as::<TrajectoryPoint>(bfr)?;
        self._participants = deserialize_message_list_as::<VehicleFormationParticipant>(bfr)?;
        self._start_time = bfr.get_f64_le();
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
