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
use crate::Header;
use crate::Message;
use crate::MessageList;
use crate::PolygonVertex;
use crate::DUNE_IMC_CONST_NULL_ID;

/// Enforced Limits
#[allow(non_camel_case_types)]
pub mod EnforcedLimitsBits {
    /// Maximum Depth Limit
    pub const ENFORCE_DEPTH: u32 = 0x01;
    /// Minimum Altitude Limit
    pub const ENFORCE_ALTITUDE: u32 = 0x02;
    /// Time Limit
    pub const ENFORCE_TIMEOUT: u32 = 0x04;
    /// Polygonal Area Limits
    pub const ENFORCE_AREA2D: u32 = 0x08;
}

/// This maneuver triggers an external controller that will guide the vehicle during a specified duration
/// of time or until it relinquishes control using (ManeuverDone). The external controller is allowed to
/// drive the vehicle only inside the specified boundaries.
#[derive(Default, Clone)]
pub struct AutonomousSection {
    /// Message Header
    pub _header: Header,
    /// WGS-84 Latitude of the initial location.
    pub _lat: f64,
    /// WGS-84 Longitude of the initial location.
    pub _lon: f64,
    /// Maneuver speed reference.
    pub _speed: f32,
    /// Speed units.
    pub _speed_units: u8,
    pub _limits: u8,
    /// Maximum depth the autonomous controller is allowed to drive to.
    pub _max_depth: f64,
    /// Minimum altitude the autonomous controller is allowed to drive to.
    pub _min_alt: f64,
    /// The time after which this maneuver should be stopped (if still active and TIMEOUT is enforced).
    pub _time_limit: f64,
    /// The boundaries of the admissable area for this autonomous section.
    pub _area_limits: MessageList<PolygonVertex>,
    /// The name of the controlling agent that will be allowed to guide the vehicle during the AutononousSection.
    pub _controller: String,
    /// Custom settings for maneuver.
    pub _custom: String,
}

impl Message for AutonomousSection {
    fn new() -> AutonomousSection {
        AutonomousSection {
            _header: Header::new(493),
            _lat: Default::default(),
            _lon: Default::default(),
            _speed: Default::default(),
            _speed_units: 0_u8,
            _limits: Default::default(),
            _max_depth: Default::default(),
            _min_alt: Default::default(),
            _time_limit: Default::default(),
            _area_limits: Default::default(),
            _controller: Default::default(),
            _custom: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        493
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        493
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
        for m in &mut self._area_limits {
            m.set_timestamp_secs(ts);
        }
    }

    fn set_source(&mut self, src: u16) {
        self.get_mut_header()._src = src;
        for m in &mut self._area_limits {
            m.set_source(src);
        }
    }

    fn set_source_ent(&mut self, src_ent: u8) {
        self.get_mut_header()._src_ent = src_ent;
        for m in &mut self._area_limits {
            m.set_source_ent(src_ent);
        }
    }

    fn set_destination(&mut self, dst: u16) {
        self.get_mut_header()._dst = dst;
        for m in &mut self._area_limits {
            m.set_destination(dst);
        }
    }

    fn set_destination_ent(&mut self, dst_ent: u8) {
        self.get_mut_header()._dst_ent = dst_ent;
        for m in &mut self._area_limits {
            m.set_destination_ent(dst_ent);
        }
    }

    fn clear(&mut self) {
        self._header = Header::new(493);
        self._lat = Default::default();
        self._lon = Default::default();
        self._speed = Default::default();
        self._speed_units = 0_u8;
        self._limits = Default::default();
        self._max_depth = Default::default();
        self._min_alt = Default::default();
        self._time_limit = Default::default();
        self._area_limits = Default::default();
        self._controller = Default::default();
        self._custom = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        46
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        message_list_serialization_size!(dyn_size, self._area_limits);
        dyn_size += self._controller.len() + 2;
        dyn_size += self._custom.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._speed);
        bfr.put_u8(self._speed_units);
        bfr.put_u8(self._limits);
        bfr.put_f64_le(self._max_depth);
        bfr.put_f64_le(self._min_alt);
        bfr.put_f64_le(self._time_limit);
        serialize_message_list!(bfr, self._area_limits);
        serialize_bytes!(bfr, self._controller.as_bytes());
        serialize_bytes!(bfr, self._custom.as_bytes());
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._speed = bfr.get_f32_le();
        self._speed_units = bfr.get_u8();
        self._limits = bfr.get_u8();
        self._max_depth = bfr.get_f64_le();
        self._min_alt = bfr.get_f64_le();
        self._time_limit = bfr.get_f64_le();
        self._area_limits = deserialize_message_list_as::<PolygonVertex>(bfr)?;
        deserialize_string!(bfr, self._controller);
        deserialize_string!(bfr, self._custom);
        Ok(())
    }
}
