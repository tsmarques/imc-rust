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

/// This message presents the estimated state of the vehicle.
/// EstimatedState is a complete description of the system
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
pub struct EstimatedState {
    /// Message Header
    pub _header: Header,
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
    /// Body-fixed frame xx axis velocity component.
    pub _u: f32,
    /// Body-fixed frame yy axis velocity component.
    pub _v: f32,
    /// Body-fixed frame zz axis velocity component.
    pub _w: f32,
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
    /// Depth, in meters. To be used by underwater vehicles. Negative
    /// values denote invalid estimates.
    pub _depth: f32,
    /// Altitude, in meters. Negative values denote invalid estimates.
    pub _alt: f32,
}

impl Message for EstimatedState {
    fn new() -> EstimatedState {
        EstimatedState {
            _header: Header::new(350),
            _lat: Default::default(),
            _lon: Default::default(),
            _height: Default::default(),
            _x: Default::default(),
            _y: Default::default(),
            _z: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _u: Default::default(),
            _v: Default::default(),
            _w: Default::default(),
            _vx: Default::default(),
            _vy: Default::default(),
            _vz: Default::default(),
            _p: Default::default(),
            _q: Default::default(),
            _r: Default::default(),
            _depth: Default::default(),
            _alt: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        350
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        350
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
        self._header = Header::new(350);
        self._lat = Default::default();
        self._lon = Default::default();
        self._height = Default::default();
        self._x = Default::default();
        self._y = Default::default();
        self._z = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._u = Default::default();
        self._v = Default::default();
        self._w = Default::default();
        self._vx = Default::default();
        self._vy = Default::default();
        self._vz = Default::default();
        self._p = Default::default();
        self._q = Default::default();
        self._r = Default::default();
        self._depth = Default::default();
        self._alt = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        88
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._lat);
        bfr.put_f64_le(self._lon);
        bfr.put_f32_le(self._height);
        bfr.put_f32_le(self._x);
        bfr.put_f32_le(self._y);
        bfr.put_f32_le(self._z);
        bfr.put_f32_le(self._phi);
        bfr.put_f32_le(self._theta);
        bfr.put_f32_le(self._psi);
        bfr.put_f32_le(self._u);
        bfr.put_f32_le(self._v);
        bfr.put_f32_le(self._w);
        bfr.put_f32_le(self._vx);
        bfr.put_f32_le(self._vy);
        bfr.put_f32_le(self._vz);
        bfr.put_f32_le(self._p);
        bfr.put_f32_le(self._q);
        bfr.put_f32_le(self._r);
        bfr.put_f32_le(self._depth);
        bfr.put_f32_le(self._alt);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._lat = bfr.get_f64_le();
        self._lon = bfr.get_f64_le();
        self._height = bfr.get_f32_le();
        self._x = bfr.get_f32_le();
        self._y = bfr.get_f32_le();
        self._z = bfr.get_f32_le();
        self._phi = bfr.get_f32_le();
        self._theta = bfr.get_f32_le();
        self._psi = bfr.get_f32_le();
        self._u = bfr.get_f32_le();
        self._v = bfr.get_f32_le();
        self._w = bfr.get_f32_le();
        self._vx = bfr.get_f32_le();
        self._vy = bfr.get_f32_le();
        self._vz = bfr.get_f32_le();
        self._p = bfr.get_f32_le();
        self._q = bfr.get_f32_le();
        self._r = bfr.get_f32_le();
        self._depth = bfr.get_f32_le();
        self._alt = bfr.get_f32_le();
        Ok(())
    }
}
