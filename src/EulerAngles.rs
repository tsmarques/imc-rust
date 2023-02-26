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

/// Report of spatial orientation according to SNAME's notation
/// (1950).
#[derive(Default, Clone)]
pub struct EulerAngles {
    /// Message Header
    pub _header: Header,
    /// The device time.
    pub _time: f64,
    /// Rotation around the vehicle longitudinal axis.
    pub _phi: f64,
    /// Rotation around the vehicle lateral or transverse axis.
    pub _theta: f64,
    /// Rotation around the vehicle vertical axis. A value of 0 means
    /// the vehicle is oriented towards true north. In cases where the
    /// sensor cannot measure the true heading, this field will have
    /// the same value as Yaw (Magnetic).
    pub _psi: f64,
    /// Rotation around the vehicle vertical axis. A value of 0 means
    /// the vehicle is oriented towards magnetic north. In cases where
    /// the sensor cannot measure the magnetic heading, this field
    /// will have the same value as Yaw (True).
    pub _psi_magnetic: f64,
}

impl Message for EulerAngles {
    fn new() -> EulerAngles {
        

        EulerAngles {
            _header: Header::new(254),
            _time: Default::default(),
            _phi: Default::default(),
            _theta: Default::default(),
            _psi: Default::default(),
            _psi_magnetic: Default::default(),
        }
    }

    #[inline(always)]
    fn static_id() -> u16 {
        254
    }

    #[inline(always)]
    fn id(&self) -> u16 {
        254
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
        self._header = Header::new(254);
        self._time = Default::default();
        self._phi = Default::default();
        self._theta = Default::default();
        self._psi = Default::default();
        self._psi_magnetic = Default::default()
    }

    #[inline(always)]
    fn fixed_serialization_size(&self) -> usize {
        40
    }

    #[inline(always)]
    fn dynamic_serialization_size(&self) -> usize {
        0
    }

    fn serialize_fields(&self, bfr: &mut bytes::BytesMut) {
        bfr.put_f64_le(self._time);
        bfr.put_f64_le(self._phi);
        bfr.put_f64_le(self._theta);
        bfr.put_f64_le(self._psi);
        bfr.put_f64_le(self._psi_magnetic);
    }

    fn deserialize_fields(&mut self, bfr: &mut dyn bytes::Buf) -> Result<(), ImcError> {
        self._time = bfr.get_f64_le();
        self._phi = bfr.get_f64_le();
        self._theta = bfr.get_f64_le();
        self._psi = bfr.get_f64_le();
        self._psi_magnetic = bfr.get_f64_le();
        Ok(())
    }
}
