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

/// Delayed Behavior
#[allow(non_camel_case_types)]
pub enum DelayedBehaviorEnum {
    /// Resume
    DBEH_RESUME = 0,
    /// Skip
    DBEH_SKIP = 1,
    /// Fail
    DBEH_FAIL = 2,
}

/// This maneuver is used to command the vehicle to arrive at some destination at
/// a specified absolute time.
/// The vehicle's speed will vary according to environment conditions and/or maneuver start time.
#[derive(Default, Clone)]
pub struct ScheduledGoto {
    /// Message Header
    pub _header: Header,
    /// Unix timestamp, in seconds, for the arrival at the destination.
    pub _arrival_time: f64,
    /// WGS-84 Latitude of target waypoint.
    pub _lat: f64,
    /// WGS-84 Longitude of target waypoint.
    pub _lon: f64,
    /// Target reference in the z axis. Use z_units to specify
    /// whether z represents depth, altitude or other.
    pub _z: f32,
    /// Units of the destination z reference.
    pub _z_units: u8,
    /// Z reference to use while travelling to the destination.
    pub _travel_z: f32,
    /// Z reference units to use while travelling to the destination.
    pub _travel_z_units: u8,
    /// What to do if the vehicle fails to arrive before or at the requested time.
    pub _delayed: u8,
}

impl Message for ScheduledGoto {
    fn new() -> ScheduledGoto {
        ScheduledGoto {
            _header: Header::new(487),
            _arrival_time: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _z: Default::default(),
            _z_units: 0_u8,
            _travel_z: Default::default(),
            _travel_z_units: 0_u8,
            _delayed: 0_u8,
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        487
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        487
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
        self._header = Header::new(487);
        self._arrival_time = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._z = Default::default();
        self._z_units = 0_u8;
        self._travel_z = Default::default();
        self._travel_z_units = 0_u8;
        self._delayed = 0_u8
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        35
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._arrival_time);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._z);
        bfr.put_u8(self._z_units);
        bfr.put_f32_le(self._travel_z);
        bfr.put_u8(self._travel_z_units);
        bfr.put_u8(self._delayed);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._arrival_time = bfr.get_f64_le();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._z = bfr.get_f32_le();
        self._z_units = bfr.get_u8();
        self._travel_z = bfr.get_f32_le();
        self._travel_z_units = bfr.get_u8();
        self._delayed = bfr.get_u8();
        Ok(())
    }
}
