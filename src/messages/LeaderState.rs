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

/// Action on the leader state
#[allow(non_camel_case_types)]
pub enum ActionontheleaderstateEnum {
    /// Request
    OP_REQUEST = 0,
    /// Set
    OP_SET = 1,
    /// Report
    OP_REPORT = 2,
}

/// This message defines the formation leader state.
/// LeaderState is a complete description of the leader state
/// in terms of parameters such as position, orientation and
/// velocities at a particular moment in time.
/// The system position is given by a North-East-Down (NED)
/// local tangent plane displacement (x, y, z) relative to
/// an absolute WGS-84 coordinate (latitude, longitude,
/// height above ellipsoid).
/// The symbols for position and attitude as well as linear and
/// angular velocities were chosen according to SNAME's notation (1950).
/// The body-fixed reference frame and Euler angles are depicted
/// next:
/// .. figure:: ../images/euler-lauv.png
/// :align:  center
/// Euler angles
#[derive(Default, Clone)]
pub struct LeaderState {
    /// Message Header
    pub _header: Header,
    /// Name for the formation group.
    pub _group_name: String,
    /// Action on the formation leader state variables
    pub _op: u8,
    /// WGS-84 Latitude.
    pub _lat: f64,
    /// WGS-84 Longitude.
    pub _lon: f64,
    /// Height above the WGS-84 ellipsoid.
    pub _height: f32,
    /// The North offset of the North/East/Down field with respect to
    /// LLH.
    pub _x: f32,
    /// The East offset of the North/East/Down field with respect to
    /// LLH.
    pub _y: f32,
    /// The Down offset of the North/East/Down field with respect to
    /// LLH.
    pub _z: f32,
    /// The phi Euler angle from the vehicle's attitude.
    pub _phi: f32,
    /// The theta Euler angle from the vehicle's attitude.
    pub _theta: f32,
    /// The psi Euler angle from the vehicle's attitude.
    pub _psi: f32,
    /// Ground Velocity xx axis velocity component.
    pub _vx: f32,
    /// Ground Velocity yy axis velocity component.
    pub _vy: f32,
    /// Ground Velocity zz axis velocity component.
    pub _vz: f32,
    /// The angular velocity over body-fixed xx axis (roll).
    pub _p: f32,
    /// The angular velocity over body-fixed yy axis (pitch).
    pub _q: f32,
    /// The angular velocity over body-fixed zz axis (yaw).
    pub _r: f32,
    /// Stream Velocity xx axis velocity component.
    pub _svx: f32,
    /// Stream Velocity yy axis velocity component.
    pub _svy: f32,
    /// Stream Velocity zz axis velocity component.
    pub _svz: f32,
}

impl Message for LeaderState {
    fn new() -> LeaderState {
        LeaderState {
            _header: Header::new(563),
            _group_name: Default::default(),
            _op: Default::default(),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _vx: Default::default(),
            _vy: Default::default(),
            _vz: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _svx: Default::default(),
            _svy: Default::default(),
            _svz: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        563
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        563
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
        self._header = Header::new(563);
        self._group_name = Default::default();
        self._op = Default::default();
        self._lat = Default::default();
        self._lon = Default::default();
        self._height = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._vx = Default::default();
        self._vy = Default::default();
        self._vz = Default::default();
        self._p = Default::default();
        self._q = Default::default();
        self._r = Default::default();
        self._svx = Default::default();
        self._svy = Default::default();
        self._svz = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        81
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        let mut dyn_size: usize = 0;
        dyn_size += self._group_name.len() + 2;

        dyn_size
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        serialize_bytes!(bfr, self._group_name.as_bytes());
        bfr.put_u8(self._op);
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._vx);
        bfr.put_f32_le(self._vy);
        bfr.put_f32_le(self._vz);
        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._q);
        bfr.put_f32_le(self._r);
        bfr.put_f32_le(self._svx);
        bfr.put_f32_le(self._svy);
        bfr.put_f32_le(self._svz);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        deserialize_string!(bfr, self._group_name);
        self._op = bfr.get_u8();
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height = bfr.get_f32_le();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._vx = bfr.get_f32_le();
        self._vy = bfr.get_f32_le();
        self._vz = bfr.get_f32_le();
        self._p = bfr.get_f32_le();
        self._q = bfr.get_f32_le();
        self._r = bfr.get_f32_le();
        self._svx = bfr.get_f32_le();
        self._svy = bfr.get_f32_le();
        self._svz = bfr.get_f32_le();
        Ok(())
    }
}
