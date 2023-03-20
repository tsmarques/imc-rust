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

/// Action on the Vehicle Simulation Parameters
#[allow(non_camel_case_types)]
pub enum ActionontheVehicleSimulationParametersEnum {
    /// Request
    OP_REQUEST = 0,
    /// Set
    OP_SET = 1,
    /// Report
    OP_REPORT = 2,
}

/// Vehicle dynamics parameters for 3DOF, 4DOF or 5DOF simulations.
#[derive(Default, Clone)]
pub struct DynamicsSimParam {
    /// Message Header
    pub _header: Header,
    /// Action on the vehicle simulation parameters for the formation control
    pub _op: u8,
    /// Proportional gain from the TAS (True Airspeed) error to the longitudinal acceleration.
    pub _tas2acc_pgain: f32,
    /// Proportional gain from the bank angle error to the bank angular rate.
    pub _bank2p_pgain: f32,
}

impl Message for DynamicsSimParam {
    fn new() -> DynamicsSimParam {
        DynamicsSimParam {
            _header: Header::new(53),
            _op: Default::default(),
            _tas2acc_pgain: Default::default(),
            _bank2p_pgain: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        53
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        53
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
        self._header = Header::new(53);
        self._op = Default::default();
        self._tas2acc_pgain = Default::default();
        self._bank2p_pgain = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        9
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._op);
        bfr.put_f32_le(self._tas2acc_pgain);
        bfr.put_f32_le(self._bank2p_pgain);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._op = bfr.get_u8();
        self._tas2acc_pgain = bfr.get_f32_le();
        self._bank2p_pgain = bfr.get_f32_le();
        Ok(())
    }
}
