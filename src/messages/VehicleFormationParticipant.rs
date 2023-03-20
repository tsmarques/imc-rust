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

/// Definition of a vehicle participant in a VehicleFormation maneuver.
#[derive(Default, Clone)]
pub struct VehicleFormationParticipant {
    /// Message Header
    pub _header: Header,
    /// IMC address of vehicle.
    pub _vid: u16,
    /// Distance that the system must respect along the xx axis.
    pub _off_x: f32,
    /// Distance that the system must respect along the yy axis.
    pub _off_y: f32,
    /// Distance that the system must respect along the zz axis.
    pub _off_z: f32,
}

impl Message for VehicleFormationParticipant {
    fn new() -> VehicleFormationParticipant {
        VehicleFormationParticipant {
            _header: Header::new(467),
            _vid: Default::default(),
            _off_x: Default::default(),
            _off_y: Default::default(),
            _off_z: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        467
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        467
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
        self._header = Header::new(467);
        self._vid = Default::default();
        self._off_x = Default::default();
        self._off_y = Default::default();
        self._off_z = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        14
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u16_le(self._vid);
        bfr.put_f32_le(self._off_x);
        bfr.put_f32_le(self._off_y);
        bfr.put_f32_le(self._off_z);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._vid = bfr.get_u16_le();
        self._off_x = bfr.get_f32_le();
        self._off_y = bfr.get_f32_le();
        self._off_z = bfr.get_f32_le();
        Ok(())
    }
}
