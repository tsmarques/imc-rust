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

/// Flags
#[allow(non_camel_case_types)]
pub mod FlagsBits {
    /// Value of x is meaningful
    pub const FL_X: u32 = 0x0001;
    /// Value of y is meaningful
    pub const FL_Y: u32 = 0x0002;
    /// Value of z is meaningful
    pub const FL_Z: u32 = 0x0004;
    /// Value of vx is meaningful
    pub const FL_VX: u32 = 0x0008;
    /// Value of vy is meaningful
    pub const FL_VY: u32 = 0x0010;
    /// Value of vz is meaningful
    pub const FL_VZ: u32 = 0x0020;
    /// Value of ax is meaningful
    pub const FL_AX: u32 = 0x0040;
    /// Value of ay is meaningful
    pub const FL_AY: u32 = 0x0080;
    /// Value of az is meaningful
    pub const FL_AZ: u32 = 0x0100;
}

/// Position, velocity and acceleration setpoints in NED
#[derive(Default, Clone)]
pub struct DesiredLinearState {
    /// Message Header
    pub _header: Header,
    /// Desired pos in x.
    pub _x: f64,
    /// Desired pos in y.
    pub _y: f64,
    /// Desired pos in z.
    pub _z: f64,
    /// Desired speed along NED x axis.
    pub _vx: f64,
    /// Desired speed along NED y axis.
    pub _vy: f64,
    /// Desired speed along NED z axis.
    pub _vz: f64,
    /// Desired acceleration along NED x axis.
    pub _ax: f64,
    /// Desired acceleration along NED y axis.
    pub _ay: f64,
    /// Desired acceleration along NED z axis.
    pub _az: f64,
    /// Setpoint Flags
    pub _flags: u16,
}

impl Message for DesiredLinearState {
    fn new() -> DesiredLinearState {
        DesiredLinearState {
            _header: Header::new(414),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _vx: Default::default(),
            _vy: Default::default(),
            _vz: Default::default(),
            _ax: Default::default(),
            _ay: Default::default(),
            _az: Default::default(),
            _flags: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        414
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        414
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
        self._header = Header::new(414);
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._vx = Default::default();
        self._vy = Default::default();
        self._vz = Default::default();
        self._ax = Default::default();
        self._ay = Default::default();
        self._az = Default::default();
        self._flags = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        74
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._x);
        bfr.put_f64_le(self._y);
        bfr.put_f64_le(self._z);
        bfr.put_f64_le(self._vx);
        bfr.put_f64_le(self._vy);
        bfr.put_f64_le(self._vz);
        bfr.put_f64_le(self._ax);
        bfr.put_f64_le(self._ay);
        bfr.put_f64_le(self._az);
        bfr.put_u16_le(self._flags);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._x = bfr.get_f64_le();
        self._y = bfr.get_f64_le();
        self._z = bfr.get_f64_le();
        self._vx = bfr.get_f64_le();
        self._vy = bfr.get_f64_le();
        self._vz = bfr.get_f64_le();
        self._ax = bfr.get_f64_le();
        self._ay = bfr.get_f64_le();
        self._az = bfr.get_f64_le();
        self._flags = bfr.get_u16_le();
        Ok(())
    }
}
