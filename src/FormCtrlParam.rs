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

/// Action
#[allow(non_camel_case_types)]
pub enum ActionEnum {
    /// Request
    OP_REQ = 0,
    /// Set
    OP_SET = 1,
    /// Report
    OP_REP = 2,
}

/// Formation controller paramenters, as: trajectory gains, control boundary layer thickness, and formation shape gains.
#[derive(Default, Clone)]
pub struct FormCtrlParam {
    /// Message Header
    pub _header: Header,
    /// Action on the vehicle formation control parameters.
    pub _Action: u8,
    /// Trajectory gain over the vehicle longitudinal direction.
    pub _LonGain: f32,
    /// Trajectory gain over the vehicle lateral direction.
    pub _LatGain: f32,
    /// Control sliding surface boundary layer thickness.
    pub _BondThick: u32,
    /// Formation shape gain (absolute vehicle position tracking).
    /// Leader control importance gain (relative to the sum of every other formation vehicle).
    pub _LeadGain: f32,
    /// Collision avoidance and formation shape gain (position tracking relative to the other formation vehicles).
    /// Individual vehicle importance gain (relative to the leader), when the relative position or the velocity state indicate higher probability of collision.
    pub _DeconflGain: f32,
}

impl Message for FormCtrlParam {
    fn new() -> FormCtrlParam {
        

        FormCtrlParam {
            _header: Header::new(820),
            _Action: Default::default(),
            _LonGain: Default::default(),
            _LatGain: Default::default(),
            _BondThick: Default::default(),
            _LeadGain: Default::default(),
            _DeconflGain: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        820
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        820
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
        self._header = Header::new(820);
        self._Action = Default::default();
        self._LonGain = Default::default();
        self._LatGain = Default::default();
        self._BondThick = Default::default();
        self._LeadGain = Default::default();
        self._DeconflGain = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        21
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_u8(self._Action);
        bfr.put_f32_le(self._LonGain);
        bfr.put_f32_le(self._LatGain);
        bfr.put_u32_le(self._BondThick);
        bfr.put_f32_le(self._LeadGain);
        bfr.put_f32_le(self._DeconflGain);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._Action = bfr.get_u8();
        self._LonGain = bfr.get_f32_le();
        self._LatGain = bfr.get_f32_le();
        self._BondThick = bfr.get_u32_le();
        self._LeadGain = bfr.get_f32_le();
        self._DeconflGain = bfr.get_f32_le();
        Ok(())
    }
}
